use std::{
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
        let ready = Arc::new(AtomicBool::new(false));
        let handle = Self {
            chan: send,
            ready: ready.clone(),
        };
        spawn(move || run_handler(recv, ready));
        handle
    }

    fn submit(&self, stream: TcpStream) -> Result<(), ThreadPoolError> {
        if self.ready.load(Ordering::Relaxed) {
            self.ready.store(false, Ordering::Relaxed);
            self.chan
                .send(stream)
                .map_err(|_| ThreadPoolError::HandlerThreadFailed)
        } else {
            Err(ThreadPoolError::ThreadNotReady)
        }
    }
}

fn run_handler(chan: Receiver<TcpStream>, ready: Arc<AtomicBool>) {
    ready.store(true, Ordering::Relaxed);
    while let Ok(stream) = chan.recv() {
        todo!("handle");
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

    fn submit(&mut self, stream: TcpStream) -> Result<(), ThreadPoolError> {
        if let Some(handle) = self.find_ready() {
            handle
                .chan
                .send(stream)
                .map_err(|_| ThreadPoolError::HandlerThreadFailed)
        } else if self.handles.len() < self.max_threads {
            let handle = ThreadHandle::spawn();
            let result = handle.chan.send(stream);
            self.handles.push(handle);
            result.map_err(|_| ThreadPoolError::HandlerThreadFailed)
        } else {
            Err(ThreadPoolError::NoThreadsAvailable)
        }
    }
}

fn reply_error(error: ThreadPoolError) {
    match error {
        ThreadPoolError::HandlerThreadFailed | ThreadPoolError::ThreadNotReady => todo!("500"),
        ThreadPoolError::NoThreadsAvailable => todo!("503"),
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), 2345))?;
    let mut thread_pool = ThreadPool::new();
    loop {
        let (stream, _addr) = listener.accept()?;
        if let Err(error) = thread_pool.submit(stream) {
            reply_error(error);
        }
    }
}
