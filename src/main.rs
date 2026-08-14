use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, SyncSender, sync_channel},
    },
    thread::spawn,
};

struct ThreadHandle {
    chan: SyncSender<TcpStream>,
    ready: Arc<AtomicBool>,
}

impl ThreadHandle {
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

fn run_handler(chan: Receiver<TcpStream>, ready: Arc<AtomicBool>) {
    while let Ok(mut stream) = chan.recv() {
        if let Err(e) = write_stream(&mut stream, 200, "hello world\n") {
            eprintln!("Error writing in handler: {e}");
        }
        ready.store(true, Ordering::Relaxed);
    }
}

struct ThreadPool {
    max_threads: usize,
    handles: Vec<ThreadHandle>,
}

enum ThreadPoolError {
    NoThreadsAvailable,
    HandlerThreadFailed,
    ThreadNotReady,
}

impl ThreadPool {
    fn new() -> Self {
        Self {
            max_threads: 32,
            handles: Vec::new(),
        }
    }

    fn find_ready(&self) -> Option<&ThreadHandle> {
        self.handles
            .iter()
            .find(|handle| handle.ready.load(Ordering::Relaxed))
    }

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

fn write_stream(
    stream: &mut TcpStream,
    status: u16,
    text: impl AsRef<[u8]>,
) -> std::io::Result<()> {
    let data = text.as_ref();
    write!(stream, "HTTP/1.1 {status} Reason Phrase\r\n")?;
    write!(stream, "Content-Type: text/plain\r\n")?;
    write!(stream, "Content-Length: {}\r\n\r\n", data.len())?;
    stream.write_all(data)?;
    Ok(())
}

fn reply_error(error: ThreadPoolError, mut stream: TcpStream) {
    let (status, message) = match error {
        ThreadPoolError::HandlerThreadFailed => (500, "handler thread failed"),
        ThreadPoolError::ThreadNotReady => (500, "thread not ready"),
        ThreadPoolError::NoThreadsAvailable => (503, "no threads available"),
    };
    if let Err(e) = write_stream(&mut stream, status, format!("{message}\n")) {
        eprintln!("Write failed in reply_error: {e}");
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(SocketAddr::new(
        Ipv4Addr::new(0, 0, 0, 0).into(),
        2345,
    ))?;
    let mut thread_pool = ThreadPool::new();
    loop {
        let (stream, _addr) = listener.accept()?;
        if let Err((error, stream)) = thread_pool.submit(stream) {
            reply_error(error, stream);
        }
    }
}
