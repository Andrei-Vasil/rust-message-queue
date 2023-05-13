use std::sync::Arc;
use super::shared_memory::SharedMemory;

pub struct QueueManager {
    shared_memory: Arc<SharedMemory>
}

impl QueueManager {
    pub fn new(shared_memory: Arc<SharedMemory>) -> QueueManager {
        QueueManager { shared_memory }
    }

    pub fn createQueueChannel(&self) {

    }

    pub fn removeQueueChannel(&self) {

    }

    pub fn publishMessage(&self) {

    }

    pub fn retrieveMessage(&self) -> i32 {
        69
    }
}