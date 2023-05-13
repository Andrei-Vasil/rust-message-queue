use std::{net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::JoinHandle, io::{Read, Write}};
use crate::pub_sub_lib::{topic_manager::TopicManager, subscription_manager::SubscriptionManager, queue_manager::QueueManager};

use super::http_request_extracter::{extract_request, ActionType, Request};

pub struct HttpRequestHandler {
    topic_manager: Arc<TopicManager>,
    subscription_manager: Arc<SubscriptionManager>,
    queue_manager: Arc<QueueManager>
}

pub struct HttpRequestHandlerWrapper {
    http_request_handler: Arc<HttpRequestHandler>
}

impl HttpRequestHandler {
    pub fn new(topic_manager: Arc<TopicManager>, subscription_manager: Arc<SubscriptionManager>, queue_manager: Arc<QueueManager>) -> HttpRequestHandler {
        HttpRequestHandler { topic_manager, subscription_manager, queue_manager }
    }

    fn process_request(&self, request: Request) -> String {
        if request.action == ActionType::NewTopic {
            self.topic_manager.new_topic();
            return String::from("HTTP/1.1 200 OK\r\n\r\n");
        }
        if request.action == ActionType::RemoveTopic {
            self.topic_manager.remove_topic();
            return String::from("HTTP/1.1 200 OK\r\n\r\n");
        }
        if request.action == ActionType::Subscribe {
            self.subscription_manager.subscribe();
            return String::from("HTTP/1.1 200 OK\r\n\r\n");
        }
        if request.action == ActionType::Unsubscribe {
            self.subscription_manager.unsubscribe();
            return String::from("HTTP/1.1 200 OK\r\n\r\n");
        }
        if request.action == ActionType::Publish {
            self.queue_manager.publishMessage();
            return String::from("HTTP/1.1 200 OK\r\n\r\n");
        }
        if request.action == ActionType::Retrieve {
            let item: i32 = self.queue_manager.retrieveMessage();
            return format!("HTTP/1.1 200 OK\r\n\r\n{:?}", item);
        }
        String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n")
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = vec![0; 4096];
        stream.read(&mut buffer).unwrap();
        
        let request = extract_request(buffer);
        let response = self.process_request(request);
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }
}

impl HttpRequestHandlerWrapper {
    pub fn new(http_request_handler: Arc<HttpRequestHandler>) -> HttpRequestHandlerWrapper {
        HttpRequestHandlerWrapper { http_request_handler }
    }

    pub fn run(&self) {
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
            let http_request_handler_copy = Arc::clone(&self.http_request_handler);
            let handle = std::thread::spawn(move || {
                http_request_handler_copy.handle_connection(stream);
            });
            handles.lock().unwrap().push(handle);
        }

        daemon_handle.join().unwrap();
    }
}