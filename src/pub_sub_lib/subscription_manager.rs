use std::sync::Arc;
use super::queue_manager::QueueManager;

pub struct SubscriptionManager {
    queue_manager: Arc<QueueManager>
}

impl SubscriptionManager {
    pub fn new(queue_manager: Arc<QueueManager>) ->  SubscriptionManager {
        SubscriptionManager { queue_manager }
    }

    pub fn subscribe(&self, topic: &String) -> Result<String, String> {
        match self.queue_manager.create_queue_channel(topic) {
            Ok(id) => Ok(format!("{id}")),
            Err(err) => Err(err)
        }
    }

    pub fn unsubscribe(&self, topic: &String, id: i32) -> Result<String, String> {
        self.queue_manager.remove_queue_channel(topic, id)
    }
}
