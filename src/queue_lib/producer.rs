use std::sync::{mpsc, Arc};
use crate::benchmark_lib::benchmark::{set_publish_over, count_producer_throughput};
use super::{queue::{Queue}, worker::Worker};

pub struct Producer<T> {
    queue: Arc<Queue<T>>,
}

impl<T> Producer<T> 
where T: 'static + Send {
    pub fn new(queue: Arc<Queue<T>>) -> Self {
        Self { queue }
    }

    pub fn push(&self, message: T, benchmark_id: usize) {
        let message_queue = self.queue.get_message_queue();
        let pop_condvar = self.queue.get_pop_condvar();
        let (tx_thread_handler, rx_thread_handler) = mpsc::channel();
        
        let worker = Worker::new(
            move || {
                let mut handle = message_queue.lock().unwrap();
                handle.push_back(message);
                pop_condvar.notify_one();
                set_publish_over(benchmark_id);
                count_producer_throughput();
                match tx_thread_handler.send(true) {
                    Ok(_) => {}
                    Err(err) => { panic!("{:?}", err.to_string()); }
                };
            },
        );
        self.queue.add_worker(worker, rx_thread_handler);
    }
}

impl<T> Drop for Producer<T> {
    fn drop(&mut self) {}
}