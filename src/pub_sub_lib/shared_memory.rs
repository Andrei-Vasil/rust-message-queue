use std::{sync::{Arc, Mutex}, collections::{HashMap, HashSet}};

use crate::queue_lib::queue::Queue;

pub struct SharedMemory {
    pub topics: Arc<Mutex<HashSet<String>>>,
    pub queue_channels: Arc<Mutex<HashMap<String, HashMap<i32, Queue<i32>>>>>,
    pub max_id_4_topic:  Arc<Mutex<HashMap<String, i32>>>
}

impl SharedMemory {
    pub fn new() -> SharedMemory {
        SharedMemory { 
            topics: Arc::new(Mutex::new(HashSet::new())), 
            queue_channels: Arc::new(Mutex::new(HashMap::new())), 
            max_id_4_topic: Arc::new(Mutex::new(HashMap::new())) 
        }
    }
}
