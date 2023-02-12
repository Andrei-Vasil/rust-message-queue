use std::{
    thread, 
    sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}}, collections::VecDeque
};

pub struct Queue {
    max_worker_id: usize,
    workers: Arc<Mutex<Vec<(Worker, Receiver<bool>)>>>,
    message_queue: Arc<Mutex<VecDeque<i32>>>,
    daemon_tx: Sender<bool>,
    daemon_handle: Option<thread::JoinHandle<()>>,
}

impl Queue {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let mut queue = Self {
            max_worker_id: 0,
            workers: Arc::new(Mutex::new(Vec::new())),
            message_queue: Arc::new(Mutex::new(VecDeque::new())),
            daemon_tx: tx,
            daemon_handle: None,
        };
        queue.daemon_handle = queue.init_daemon_thread(rx);
        queue
    }

    pub fn init_daemon_thread(&mut self, rx: Receiver<bool>) -> Option<thread::JoinHandle<()>> {
        let workers_clone = self.workers.clone();
        Some(thread::spawn(move || loop {
            match rx.try_recv() {
                Ok(_) => {
                    break;
                },
                Err(_) => {},
            };

            let mut guard = workers_clone.lock().unwrap();
            let workers = &mut *guard;
            workers.retain(|(worker, rx)| {
                match rx.try_recv() {
                    Ok(_) => {
                        drop(worker);
                        false
                    },
                    Err(_) =>true,
                }
            });
        }))
    }

    pub fn push(&mut self, message: i32) {
        let message_queue = self.message_queue.clone();
        let (tx_thread_handler, rx_thread_handler) = mpsc::channel();

        self.max_worker_id += 1;
        let id = self.max_worker_id;
        self.workers.lock().unwrap().push((
            Worker::new(
                self.max_worker_id, 
                move || {
                    println!("{:?}", id);
                    let mut handle = message_queue.lock().unwrap();
                    handle.push_back(message);
                    tx_thread_handler.send(true);
                },
            ), 
            rx_thread_handler,
        ));
    }

    pub fn pop(&mut self) -> Option<i32> {
        let message_queue = self.message_queue.clone();
        let (tx_thread_handler, rx_thread_handler) = mpsc::channel();
        let (tx, rx) = mpsc::channel();
        
        self.max_worker_id += 1;
        let id = self.max_worker_id;
        self.workers.lock().unwrap().push((
            Worker::new(
                self.max_worker_id, 
                move || {
                    println!("{:?}", id);
                    let mut handle = message_queue.lock().unwrap();
                    tx.send(handle.pop_front());
                    tx_thread_handler.send(true);
                },
            ),
            rx_thread_handler,
        ));
        rx.recv().unwrap()
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        self.daemon_tx.send(true);
        if let Some(thread) = self.daemon_handle.take() {
            thread.join().unwrap();
        }
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