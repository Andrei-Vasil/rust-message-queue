use std::{net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::JoinHandle, io::{Read, Write}};
use crate::pub_sub_lib::{topic_manager::TopicManager, subscription_manager::SubscriptionManager, queue_manager::QueueManager};

use super::http_request_extracter::{extract_request, ActionType, Request};

pub struct HttpRequestHandler {
    topic_manager: Arc<TopicManager>,
    subscription_manager: Arc<SubscriptionManager>,
    queue_manager: Arc<QueueManager>,
}

impl HttpRequestHandler {
    pub fn new(topic_manager: Arc<TopicManager>, subscription_manager: Arc<SubscriptionManager>, queue_manager: Arc<QueueManager>) -> HttpRequestHandler {
        HttpRequestHandler { topic_manager, subscription_manager, queue_manager }
    }

    fn new_topic(&self, request: Request) -> String {
        if request.params.len() != 1 {
            return "HTTP/1.1 404 NOT FOUND\r\n\r\nInvalid path\r\n".to_string();
        }
        let topic = &request.params[0];
        match self.topic_manager.new_topic(topic) {
            Ok(message) => format!("HTTP/1.1 200 OK\r\n\r\n{message}\r\n"),
            Err(err) => format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{err}\r\n")
        }
    }
    
    fn remove_topic(&self, request: Request) -> String {
        if request.params.len() != 1 {
            return "HTTP/1.1 404 NOT FOUND\r\n\r\nInvalid path\r\n".to_string();
        }
        let topic = &request.params[0];
        match self.topic_manager.remove_topic(topic) {
            Ok(message) => format!("HTTP/1.1 200 OK\r\n\r\n{message}\r\n"),
            Err(err) => format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{err}\r\n")
        }
    }

    fn subscribe(&self, request: Request) -> String {
        if request.params.len() != 1 {
            return "HTTP/1.1 404 NOT FOUND\r\n\r\nInvalid path\r\n".to_string();
        }
        let topic = &request.params[0];
        match self.subscription_manager.subscribe(topic) {
            Ok(message) => format!("HTTP/1.1 200 OK\r\n\r\n{message}\r\n"),
            Err(err) => format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{err}\r\n")
        }
    }

    fn unsubscribe(&self, request: Request) -> String {
        if request.params.len() != 2 {
            return "HTTP/1.1 404 NOT FOUND\r\n\r\nInvalid path\r\n".to_string();
        }
        let topic = &request.params[0];
        let id = request.params[1].parse::<i32>().unwrap();
        match self.subscription_manager.unsubscribe(topic, id) {
            Ok(message) => format!("HTTP/1.1 200 OK\r\n\r\n{message}\r\n"),
            Err(err) => format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{err}\r\n")
        }
    }

    fn publish(&self, request: Request) -> String {
        if request.params.len() != 1 {
            return "HTTP/1.1 404 NOT FOUND\r\n\r\nInvalid path\r\n".to_string();
        }
        let topic = &request.params[0];
        let message = request.body.get("item").unwrap().to_string().parse::<i32>().unwrap();
        match self.queue_manager.publish_message(topic, message) {
            Ok(message) => format!("HTTP/1.1 200 OK\r\n\r\n{message}\r\n"),
            Err(err) => format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{err}\r\n")
        }
    }

    fn retrieve(&self, request: Request) -> String {
        if request.params.len() != 2 {
            return "HTTP/1.1 404 NOT FOUND\r\n\r\nInvalid path\r\n".to_string();
        }
        let topic = &request.params[0];
        let id = request.params[1].parse::<i32>().unwrap();
        match self.queue_manager.retrieve_message(topic, id) {
            Ok(message) => format!("HTTP/1.1 200 OK\r\n\r\n{message}\r\n"),
            Err(err) => format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{err}\r\n")
        }
    }

    fn process_request(&self, request: Request) -> String {
        match request.action {
            ActionType::NewTopic => self.new_topic(request),
            ActionType::RemoveTopic => self.remove_topic(request),
            ActionType::Subscribe => self.subscribe(request),
            ActionType::Unsubscribe => self.unsubscribe(request),
            ActionType::Publish => self.publish(request),
            ActionType::Retrieve => self.retrieve(request),
            ActionType::NULL => String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n")
        }
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

pub struct HttpRequestHandlerWrapper {
    http_request_handler: Arc<HttpRequestHandler>
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