use std::sync::{mpsc, Arc};

use super::{queue::Queue, worker::Worker};


pub struct Consumer {
    queue: Arc<Queue>,
}

impl Consumer {
    pub fn new(queue: Arc<Queue>) -> Self {
        Self { queue }
    }

    pub fn pop(&self) -> Option<i32> {
        let message_queue = self.queue.get_message_queue();
        let pop_condvar = self.queue.get_pop_condvar();
        let (tx_thread_handler, rx_thread_handler) = mpsc::channel();
        let (tx, rx) = mpsc::channel();
        
        let id = self.queue.inc_max_worker_id();
        let worker = Worker::new(
            move || {
                println!("{:?} popper", id);
                let mut guard = message_queue.lock().unwrap();
                while guard.len() == 0 {
                    println!("{:?} waitin", id);
                    guard = pop_condvar.wait(guard).unwrap();
                }
                tx.send(guard.pop_front());
                tx_thread_handler.send(true);
            },
        );
        self.queue.add_worker(worker, rx_thread_handler);
        rx.recv().unwrap()
    }
}

impl Drop for Consumer {
    fn drop(&mut self) {}
}
