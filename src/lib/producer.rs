use std::sync::{mpsc, Arc};

use super::{queue::{Queue}, worker::Worker};

pub struct Producer {
    queue: Arc<Queue>,
}

impl Producer {
    pub fn new(queue: Arc<Queue>) -> Self {
        Self { queue }
    }

    pub fn push(&self, message: i32) {
        let message_queue = self.queue.get_message_queue();
        let pop_condvar = self.queue.get_pop_condvar();
        let (tx_thread_handler, rx_thread_handler) = mpsc::channel();
        
        let id = self.queue.inc_max_worker_id();
        let worker = Worker::new(
            move || {
                println!("{:?} pusher", id);
                let mut handle = message_queue.lock().unwrap();
                // println!("push: {:?}", message);
                handle.push_back(message);
                pop_condvar.notify_one();
                tx_thread_handler.send(true);
            },
        );
        self.queue.add_worker(worker, rx_thread_handler);
    }
}

impl Drop for Producer {
    fn drop(&mut self) {}
}