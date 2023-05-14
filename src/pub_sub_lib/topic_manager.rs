use std::{sync::Arc, collections::HashMap};
use super::shared_memory::SharedMemory;

pub struct TopicManager {
    shared_memory: Arc<SharedMemory>
}

impl TopicManager {
    pub fn new(shared_memory: Arc<SharedMemory>) -> TopicManager {
        TopicManager { shared_memory }
    }

    pub fn new_topic(&self, topic: &String) -> Result<String, String> {
        if self.exists(topic) {
            return Err(format!("Topic named {topic} already exists"));
        }
        self.shared_memory.topics.lock().unwrap().insert(topic.to_string());
        self.shared_memory.max_id_4_topic.lock().unwrap().insert(topic.clone(), 0);
        self.shared_memory.queue_channels.lock().unwrap().insert(topic.clone(), HashMap::new());
        Ok(format!("Successfully created the new topic {topic}"))
    }

    pub fn remove_topic(&self, topic: &String) -> Result<String, String> {
        if !self.exists(topic) {
            return Err(format!("There is no topic named: {topic}"));
        }
        self.shared_memory.topics.lock().unwrap().remove(topic);
        self.shared_memory.max_id_4_topic.lock().unwrap().remove(topic);
        self.shared_memory.queue_channels.lock().unwrap().remove(topic);
        Ok(format!("Successfully deleted {topic} topic"))
    }

    pub fn exists(&self, topic: &String) -> bool {
        self.shared_memory.topics.lock().unwrap().contains(topic)
    }
}