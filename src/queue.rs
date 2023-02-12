use std::{
    thread, 
    sync::{Arc, Mutex, mpsc}, collections::VecDeque
};

pub struct Queue {
    max_worker_id: usize,
    listeners: Vec<Worker>,
    message_queue: Arc<Mutex<VecDeque<i32>>>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            max_worker_id: 0,
            listeners: Vec::new(),
            message_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn push(&mut self, message: i32) {
        let message_queue = self.message_queue.clone();
        self.listeners.push(Worker::new(
            self.max_worker_id, 
            move || {
                let mut handle = message_queue.lock().unwrap();
                handle.push_back(message);
            },
        ));
    }

    pub fn pop(&mut self) -> Option<i32> {
        let message_queue = self.message_queue.clone();
        let (tx, rx) = mpsc::channel();
        self.listeners.push(Worker::new(
            self.max_worker_id, 
            move || {
                let mut handle = message_queue.lock().unwrap();
                tx.send(handle.pop_front());
            },
        ));
        rx.recv().unwrap()
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        // println!("{:?}", self.message_queue.lock().unwrap());
    }
}

struct Worker {
    id: usize, 
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new<F>(id: usize, f: F) -> Self 
    where
        F: FnOnce() + Send + 'static, 
    {
        Self { id, thread: Some(thread::spawn(f)) }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}