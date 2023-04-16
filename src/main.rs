mod queue_lib {
    pub mod queue;
    pub mod producer;
    pub mod consumer;
    mod daemon;
    mod worker;
}

mod http_comms_lib {
    pub mod request_handler;
}

use std::{sync::{Arc, Mutex}, net::{TcpListener, TcpStream}, io::{Write, Read}, thread::JoinHandle};
use queue_lib::{queue::{Queue}, consumer::Consumer, producer::Producer};
use http_comms_lib::request_handler::{extract_request, ActionType, Request};

fn pop(q: Arc<Queue<i32>>) -> i32 {
    let consumer = Consumer::new(q);
    consumer.pop().unwrap()
}

fn push(request: Request, q: Arc<Queue<i32>>) {
    let producer = Producer::new(q);
    // println!("{:?}", request);
    if request.body != serde_json::Value::Null {
        producer.push(request.body["item"].as_i64().unwrap() as i32);
        return;
    }
    producer.push(0);
}

fn handle_connection(mut stream: TcpStream, q: Arc<Queue<i32>>) {
    let mut buffer = vec![0; 4096];
    stream.read(&mut buffer).unwrap();
    
    let request = extract_request(buffer);

    if request.action == ActionType::NULL {
        stream.write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }
    if request.action == ActionType::PUSH {
        push(request, q);
        stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }
    if request.action == ActionType::POP {
        let popped_item = pop(q);
        stream.write_all(format!("HTTP/1.1 200 OK\r\n\r\n{:?}\r\n", popped_item).as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }
    stream.write_all("HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n".as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let q: Arc<Queue<i32>> = Arc::new(Queue::new());
    let listener = TcpListener::bind("0.0.0.0:5000").unwrap();
    let handles: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));
    
    let handles_copy = Arc::clone(&handles);
    let daemon_handle = std::thread::spawn(move || loop {
        if let Some(handle) = handles_copy.lock().unwrap().pop().take() {
            handle.join().unwrap();
        }
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let q_clone = q.clone();
        let handle = std::thread::spawn(move || {
            handle_connection(stream, q_clone);
        });
        handles.lock().unwrap().push(handle);
    }

    daemon_handle.join().unwrap();
}
