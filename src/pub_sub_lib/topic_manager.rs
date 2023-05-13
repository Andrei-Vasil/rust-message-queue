use std::sync::Arc;
use super::shared_memory::SharedMemory;

pub struct TopicManager {
    shared_memory: Arc<SharedMemory>
}

impl TopicManager {
    pub fn new(shared_memory: Arc<SharedMemory>) -> TopicManager {
        TopicManager { shared_memory }
    }

    pub fn new_topic(&self) {

    }

    pub fn remove_topic(&self) {

    }
}