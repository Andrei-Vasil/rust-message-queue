use std::sync::Arc;
use super::{shared_memory::SharedMemory, queue_manager::QueueManager};

pub struct SubscriptionManager {
    shared_memory: Arc<SharedMemory>,
    queue_manager: Arc<QueueManager>
}

impl SubscriptionManager {
    pub fn new(shared_memory: Arc<SharedMemory>, queue_manager: Arc<QueueManager>) ->  SubscriptionManager {
        SubscriptionManager { shared_memory, queue_manager }
    }

    pub fn subscribe(&self, topic: &String) -> Result<String, String> {
        Ok("subscribe".to_string())
    }

    pub fn unsubscribe(&self, topic: &String, id: usize) -> Result<String, String> {
        Ok("unsubscribe".to_string())
    }
}
