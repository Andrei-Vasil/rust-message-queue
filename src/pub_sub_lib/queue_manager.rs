use std::sync::Arc;
use crate::queue_lib::queue::Queue;
use super::{shared_memory::SharedMemory, topic_manager::TopicManager};

pub struct QueueManager {
    shared_memory: Arc<SharedMemory>,
    topic_manager: Arc<TopicManager>
}

impl QueueManager {
    pub fn new(shared_memory: Arc<SharedMemory>, topic_manager: Arc<TopicManager>) -> QueueManager {
        QueueManager { shared_memory, topic_manager }
    }

    pub fn create_queue_channel(&self, topic: &String) -> Result<i32, String> {
        if !self.topic_manager.exists(topic) {
            return Err(format!("There is no topic named: {topic}"));
        }
        let max_id_4_topic = &mut *self.shared_memory.max_id_4_topic.lock().unwrap();
        let id = max_id_4_topic.get(topic).cloned().unwrap();
        max_id_4_topic.insert(topic.clone(), id + 1);
        let queue_channels = &mut *self.shared_memory.queue_channels.lock().unwrap();
        let queue_channel = queue_channels.get_mut(topic).unwrap();
        queue_channel.insert(id, Queue::<i32>::new());
        Ok(id)
    }

    pub fn remove_queue_channel(&self, topic: &String, id: i32) -> Result<String, String> {
        if !self.topic_manager.exists(topic) {
            return Err(format!("There is no topic named: {topic}"));
        }
        let queue_channels = &mut *self.shared_memory.queue_channels.lock().unwrap();
        let queue_channels = queue_channels.get_mut(topic).unwrap();
        match queue_channels.remove(&id) {
            Some(_) => Ok(format!("Successfully unsubscribed id {id} from {topic} topic")),
            None => Err(format!("There is no id with specified value: {id}"))
        }
    }

    pub fn publish_message(&self, topic: &String) -> Result<String, String> {
        Ok("publish message".to_string())
    }

    pub fn retrieve_message(&self, topic: &String, id: i32) -> Result<String, String> {
        Ok(69.to_string())
    }
}