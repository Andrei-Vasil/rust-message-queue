mod lib {
    pub mod queue;
    pub mod producer;
    pub mod consumer;
    mod daemon;
    mod worker;
}
use std::{sync::{Arc}, thread, time::Duration};
use lib::{queue::{Queue}, consumer::Consumer, producer::Producer};

fn main() {
    let q = Arc::new(Queue::new());
    let q_clone = q.clone();
    let producer = Producer::new(q_clone);

    producer.push(1);
    producer.push(2);
    producer.push(3);

    let q_clone = q.clone();
    let consumer = Consumer::new(q_clone);
    let handle = thread::spawn(move || { 
        println!("{:?}", consumer.pop());
        println!("{:?}", consumer.pop());
        println!("{:?}", consumer.pop());
        println!("{:?}", consumer.pop());
    });

    thread::sleep(Duration::from_millis(200));
    producer.push(4);

    handle.join().unwrap();
}
