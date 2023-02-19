use std::sync::{mpsc, Arc};

use super::{queue::Queue, worker::Worker};


pub struct Consumer<T> {
    queue: Arc<Queue<T>>,
}

impl<T> Consumer<T>
where T: 'static + Send {
    pub fn new(queue: Arc<Queue<T>>) -> Self {
        Self { queue }
    }

    pub fn pop(&self) -> Option<T> {
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
                match tx.send(guard.pop_front()) {
                    Ok(_) => {
                        match tx_thread_handler.send(true) {
                            Ok(_) => {},
                            Err(err) => { panic!("{:?}", err.to_string()); }
                        };
                    }
                    Err(err) => { panic!("{:?}", err.to_string()); }
                };
            },
        );
        self.queue.add_worker(worker, rx_thread_handler);
        rx.recv().unwrap()
    }
}

impl<T> Drop for Consumer<T> {
    fn drop(&mut self) {}
}
