mod queue;
use std::{sync::{Mutex, Arc}, thread};

use queue::Queue;

fn main() {
    let mut handles = vec![];

    let q = Arc::new(Mutex::new(Queue::new()));
    
    // let q1 = q.clone();
    // let handle = thread::spawn(move || {
    //     q1.lock().unwrap().push(1);
    // });
    // handles.push(handle);

    let q2 = q.clone();
    let handle = thread::spawn(move || {
        q2.lock().unwrap().push(1);
        q2.lock().unwrap().push(2);
    });
    handles.push(handle);

    let q3 = q.clone();
    let handle = thread::spawn(move || {
        q3.lock().unwrap().push(3);
    });
    handles.push(handle);


    // let q4 = q.clone();
    // let handle = thread::spawn(move || {
    //     println!("{:?}", q4.lock().unwrap().pop());
    // });
    // handles.push(handle);

    let q5 = q.clone();
    let handle = thread::spawn(move || {
        println!("{:?}", q5.lock().unwrap().pop());
        println!("{:?}", q5.lock().unwrap().pop());
        println!("{:?}", q5.lock().unwrap().pop());
        println!("{:?}", q5.lock().unwrap().pop());
    });
    handles.push(handle);

    // let q6 = q.clone();
    // let handle = thread::spawn(move || {
    //     println!("{:?}", q6.lock().unwrap().pop());
    // });
    // handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
