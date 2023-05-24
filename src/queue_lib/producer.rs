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

    pub fn push(&self, message: T, benchmark_id: usize, scenario_id: Arc<String>) {
        let message_queue = self.queue.get_message_queue();
        let pop_condvar = self.queue.get_pop_condvar();
        let (tx_thread_handler, rx_thread_handler) = mpsc::channel();
        
        let scenario_id_clone_1 = Arc::clone(&scenario_id);
        let scenario_id_clone_2 = Arc::clone(&scenario_id);
        let worker = Worker::new(
            move || {
                let mut handle = message_queue.lock().unwrap();
                handle.push_back(message);
                pop_condvar.notify_one();
                set_publish_over(benchmark_id, scenario_id_clone_1);
                count_producer_throughput(scenario_id_clone_2);
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