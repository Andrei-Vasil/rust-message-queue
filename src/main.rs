mod queue;
use std::{sync::{Mutex, Arc}, thread};

use queue::Queue;

fn main() {
    let mut q = Queue::new();    
    q.push(1);
    q.push(2);
    q.push(3);
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
}
