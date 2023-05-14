
use std::{sync::{Arc, Mutex, Condvar, mpsc::{Receiver}}, collections::VecDeque};
use super::worker::Worker;
use super::daemon::Daemon;

pub struct Queue<T> {
    // max_worker_id: Mutex<usize>,
    workers: Arc<Mutex<Vec<(Worker, Receiver<bool>)>>>,
    message_queue: Arc<Mutex<VecDeque<T>>>,
    pop_condvar: Arc<Condvar>,
    daemon_thread: Daemon,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let workers = Arc::new(Mutex::new(Vec::new()));
        let workers_clone = workers.clone();
        Self {
            // max_worker_id: Mutex::new(0),
            workers: workers,
            message_queue: Arc::new(Mutex::new(VecDeque::new())),
            pop_condvar: Arc::new(Condvar::new()),
            daemon_thread: Daemon::new(workers_clone),
        }
    }

    pub fn add_worker(&self, worker: Worker, rx_thread_handler: Receiver<bool>) {
        self.workers.lock().unwrap().push((
            worker, 
            rx_thread_handler,
        ));
    }

    // pub fn inc_max_worker_id(&self) -> usize {
    //     let mut guard = self.max_worker_id.lock().unwrap();
    //     *guard += 1;
    //     *guard
    // }

    pub fn get_message_queue(&self) -> Arc<Mutex<VecDeque<T>>> {
        self.message_queue.clone()
    }

    pub fn get_pop_condvar(&self) -> Arc<Condvar> {
        self.pop_condvar.clone()
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        drop(&mut self.daemon_thread);
    }
}
