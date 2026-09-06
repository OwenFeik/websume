use std::{
    fs::File,
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    os::{fd::AsRawFd, unix::fs::MetadataExt},
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, SyncSender, sync_channel},
    },
    thread::spawn,
    time::Duration,
};

use crate::http::Status;

mod http;
mod request;
mod url;

/// Handle which holds a channel to send new connections to a handler thread.
struct ThreadHandle {
    chan: SyncSender<TcpStream>,
    ready: Arc<AtomicBool>,
}

impl ThreadHandle {
    /// Start a background handler thread and return a handle to submit
    /// connections for that thread to handle.
    fn spawn() -> Self {
        let (send, recv) = sync_channel::<TcpStream>(1);
        let ready = Arc::new(AtomicBool::new(true));
        let handle = Self {
            chan: send,
            ready: ready.clone(),
        };
        spawn(move || run_handler(recv, ready));
        handle
    }

    /// Submit a new connection for this thread to handle. If this fails because
    /// the handler thread is busy or has itself failed, returns an appropriate
    /// error and the stream back to the caller.
    fn submit(
        &self,
        stream: TcpStream,
    ) -> Result<(), (ThreadPoolError, TcpStream)> {
        if self.ready.load(Ordering::Relaxed) {
            self.ready.store(false, Ordering::Relaxed);
            if let Err(e) = self.chan.send(stream) {
                Err((ThreadPoolError::HandlerThreadFailed, e.0))
            } else {
                Ok(())
            }
        } else {
            Err((ThreadPoolError::ThreadNotReady, stream))
        }
    }
}

/// Errors that can be encountered while receiving a request from a client
/// connection.
#[expect(dead_code)] // Fields for error information.
#[derive(Debug)]
enum ReceiveError {
    Io(std::io::Error),
    Parse(request::ParseFailure),
}

/// Receive a single HTTP request from the provided stream, using buf to buffer
/// data from the stream.
fn receive_request(
    buf: &mut [u8],
    stream: &mut TcpStream,
) -> Result<request::HttpRequest, ReceiveError> {
    loop {
        let mut parser = request::RequestParser::new();
        match stream.read(buf) {
            Ok(count) => match parser.parse(&buf[0..count]) {
                request::ParseOutcome::Ongoing(p) => parser = p,
                request::ParseOutcome::Complete(request) => return Ok(request),
                request::ParseOutcome::Failed(error) => {
                    return Err(ReceiveError::Parse(error));
                }
            },
            Err(e) => {
                eprintln!("Error reading in handler: {e}");
                return Err(ReceiveError::Io(e));
            }
        }
    }
}

unsafe extern "C" {
    /// Copy count bytes from in_fd to out_fd using the sendfile syscall.
    fn sendfile64(
        out_fd: i32,
        in_fd: i32,
        offset: *mut i64,
        count: usize,
    ) -> isize;
}

/// Send the file located at `path` through `stream` using the `sendfile64`
/// syscall.
fn send_file(path: &Path, stream: &mut TcpStream) -> std::io::Result<()> {
    let file = File::open(path)?;
    let out_fd = stream.as_raw_fd();
    let in_fd = file.as_raw_fd();
    let count = file.metadata()?.size() as usize;
    write_response_headers(stream, Status::Ok, count)?;
    let ret = unsafe { sendfile64(out_fd, in_fd, std::ptr::null_mut(), count) };
    if ret <= 0 {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "sendfile64 failed",
        ))
    } else {
        Ok(())
    }
}

/// Reply to a `GET path` with the contents of the file at `path`, or an
/// appropriate error.
fn reply_file(path: &str, mut stream: TcpStream) -> std::io::Result<()> {
    let path = url::parse_path(path);
    if let Err(e) = send_file(&path, &mut stream) {
        let status = match e.kind() {
            std::io::ErrorKind::NotFound => Status::NotFound,
            _ => Status::InternalServerError,
        };
        respond_with_data(&mut stream, status, format!("error: {e}\n"))?;
    }
    Ok(())
}

/// Handle incoming connections from `chan` indefinitely. `ready` will be set to
/// true whenever a connection is finished, to indicate that this handler is
/// ready to receive a new connection.
fn run_handler(chan: Receiver<TcpStream>, ready: Arc<AtomicBool>) {
    let mut buf: Box<[u8; 4096]> = Box::new([0; 4096]);
    while let Ok(mut stream) = chan.recv() {
        if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(1))) {
            eprintln!("Error setting stream read timeout: {e}");
        } else {
            let result = match receive_request(buf.as_mut_slice(), &mut stream)
            {
                Ok(request) => reply_file(&request.path, stream),
                Err(e) => respond_with_data(
                    &mut stream,
                    Status::BadRequest,
                    format!("bad request: {e:?}\n"),
                ),
            };
            if let Err(e) = result {
                eprintln!("Error writing in handler: {e}");
            }
        }
        ready.store(true, Ordering::Relaxed);
    }
}

/// Pool of handler threads to handle incoming connections.
struct ThreadPool {
    max_threads: usize,
    handles: Vec<ThreadHandle>,
}

/// Errors that can occur when attempting to submit a new connection to a
/// handler.
enum ThreadPoolError {
    NoThreadsAvailable,
    HandlerThreadFailed,
    ThreadNotReady,
}

impl ThreadPool {
    /// Create a new thread pool.
    fn new() -> Self {
        const MAX_THREADS: usize = 32;
        Self {
            max_threads: MAX_THREADS,
            handles: Vec::new(),
        }
    }

    /// Find a thread that is ready to receive a connection and return a handle
    /// to it.
    fn find_ready(&self) -> Option<&ThreadHandle> {
        self.handles
            .iter()
            .find(|handle| handle.ready.load(Ordering::Relaxed))
    }

    /// Submit a new connection to be handled by a handler thread. Returns an
    /// error and the connection if no thread can handle the request.
    fn submit(
        &mut self,
        stream: TcpStream,
    ) -> Result<(), (ThreadPoolError, TcpStream)> {
        if let Some(handle) = self.find_ready() {
            handle.submit(stream)
        } else if self.handles.len() < self.max_threads {
            let handle = ThreadHandle::spawn();
            let result = handle.submit(stream);
            self.handles.push(handle);
            result
        } else {
            Err((ThreadPoolError::NoThreadsAvailable, stream))
        }
    }
}

/// Write out an HTTP response up to the start of the body data, providing the
/// appropriate Content-Length as specified by `length`.
fn write_response_headers(
    stream: &mut TcpStream,
    status: Status,
    length: usize,
) -> std::io::Result<()> {
    write!(stream, "HTTP/1.1 {} {status}\r\n", status.code())?;
    write!(stream, "Content-Type: text/plain\r\n")?;
    write!(stream, "Content-Length: {}\r\n\r\n", length)?;
    Ok(())
}

/// Write an HTTP response to `stream` with the provided `status` and body
/// `data`.
fn respond_with_data(
    stream: &mut TcpStream,
    status: Status,
    data: impl AsRef<[u8]>,
) -> std::io::Result<()> {
    let data = data.as_ref();
    write_response_headers(stream, status, data.len())?;
    stream.write_all(data)?;
    Ok(())
}

/// Send a response on `stream` describing the error encountered.
fn reply_error(error: ThreadPoolError, mut stream: TcpStream) {
    let (status, message) = match error {
        ThreadPoolError::HandlerThreadFailed => {
            (Status::InternalServerError, "handler thread failed")
        }
        ThreadPoolError::ThreadNotReady => {
            (Status::InternalServerError, "thread not ready")
        }
        ThreadPoolError::NoThreadsAvailable => {
            (Status::ServiceUnavailable, "no threads available")
        }
    };
    if let Err(e) =
        respond_with_data(&mut stream, status, format!("{message}\n"))
    {
        eprintln!("Write failed in reply_error: {e}");
    }
}

fn main() -> std::io::Result<()> {
    let port = 2345;
    let listener = TcpListener::bind(SocketAddr::new(
        Ipv4Addr::new(0, 0, 0, 0).into(),
        port,
    ))?;
    println!("Listening on port {port}");
    let mut thread_pool = ThreadPool::new();
    loop {
        let (stream, _addr) = listener.accept()?;
        if let Err((error, stream)) = thread_pool.submit(stream) {
            reply_error(error, stream);
        }
    }
}
