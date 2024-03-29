use std::sync::Arc;
use crate::queue_lib::{queue::Queue, producer::Producer, consumer::Consumer};
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
        queue_channel.insert(id, Arc::new(Queue::<Arc<serde_json::Value>>::new()));
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

    pub fn publish_message(&self, topic: &String, message: serde_json::Value, benchmark_id: usize, scenario_id: Arc<String>) -> Result<String, String> {
        if !self.topic_manager.exists(topic) {
            return Err(format!("There is no topic named: {topic}"));
        }
        let queue_channels = &*self.shared_memory.queue_channels.lock().unwrap();
        let topic_queue_channels = queue_channels.get(topic).unwrap();
        let message_arc = Arc::new(message);
        for (_, queue_channel) in topic_queue_channels {
            let scenario_id_clone = Arc::clone(&scenario_id);
            let producer = Producer::new(Arc::clone(queue_channel));
            producer.push(Arc::clone(&message_arc), benchmark_id, scenario_id_clone);
        }
        Ok(format!("Successfully published your message to {topic} topic"))
    }

    pub fn retrieve_message(&self, topic: &String, id: i32, scenario_id: Arc<String>) -> Result<String, String> {
        if !self.topic_manager.exists(topic) {
            return Err(format!("There is no topic named: {topic}"));
        }
        let queue_channels = &*self.shared_memory.queue_channels.lock().unwrap();
        let topic_queue_channels = queue_channels.get(topic).unwrap();
        match topic_queue_channels.get(&id) {
            Some(queue_channel) => {
                let consumer = Consumer::new(Arc::clone(queue_channel));
                let item = consumer.pop(scenario_id).unwrap();
                Ok(format!("{item}"))
            },
            None => Err(format!("There is no id with specified value: {id}"))
        }
    }
}