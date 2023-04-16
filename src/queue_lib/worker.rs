use std::thread;

pub struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new<F>(f: F) -> Self 
    where
        F: FnOnce() + Send + 'static, 
    {
        Self { thread: Some(thread::spawn(f)) }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
