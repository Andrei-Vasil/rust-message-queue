use std::{io::Read, str::FromStr, net::TcpStream};

use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ActionType {
    NewTopic,
    RemoveTopic,
    Subscribe,
    Unsubscribe,
    Publish,
    Retrieve,
    NULL
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Request {
    pub action: ActionType,
    pub params: Vec<String>,
    pub body: serde_json::Value
}

fn clear_trash_from_buffer(buffer: Vec<u8>) -> String {
    let mut http_request: String = "".to_string();
    for byte in buffer.bytes() {
        let byte_as_u8 = byte.unwrap();
        if byte_as_u8 == 0 {
            break;
        }
        http_request.push(byte_as_u8 as char);
    }
    http_request
}

fn extract_action(request_row: &str) -> (ActionType, Vec<String>) {
    let mut action: ActionType = ActionType::NULL;
    let mut params: Vec<String> = Vec::new();

    request_row.split(" ").for_each(|tiny_slice: &str| {
        if tiny_slice.starts_with("/topic") && request_row.starts_with("POST") {
            action = ActionType::NewTopic;
            let split_path: Vec<&str> = tiny_slice.split('/').collect();
            params.push(String::from_str(split_path[2]).unwrap());
        }
        if tiny_slice.starts_with("/topic") && request_row.starts_with("DELETE") {
            action = ActionType::RemoveTopic;
            let split_path: Vec<&str> = tiny_slice.split('/').collect();
            params.push(String::from_str(split_path[2]).unwrap());
        }
        if tiny_slice.starts_with("/subscription") && request_row.starts_with("POST") {
            action = ActionType::Subscribe;
            let split_path: Vec<&str> = tiny_slice.split('/').collect();
            params.push(String::from_str(split_path[2]).unwrap());
        }
        if tiny_slice.starts_with("/subscription") && request_row.starts_with("DELETE") {
            action = ActionType::Unsubscribe;
            let split_path: Vec<&str> = tiny_slice.split('/').collect();
            params.push(String::from_str(split_path[2]).unwrap());
            params.push(String::from_str(split_path[3]).unwrap());
        }
        if tiny_slice.starts_with("/publish") && request_row.starts_with("POST") {
            action = ActionType::Publish;
            let split_path: Vec<&str> = tiny_slice.split('/').collect();
            params.push(String::from_str(split_path[2]).unwrap());
        }
        if tiny_slice.starts_with("/subscription") && request_row.starts_with("GET") {
            action = ActionType::Retrieve;
            let split_path: Vec<&str> = tiny_slice.split('/').collect();
            params.push(String::from_str(split_path[2]).unwrap());
            params.push(String::from_str(split_path[3]).unwrap());
        }
    });
    (action, params)
}

fn get_body(stream: &mut TcpStream) -> String {
    stream.set_nonblocking(true).unwrap();

    let mut request_as_str: String = String::new();
    let mut buffer = vec![0; 5 * 1024 * 1024];
    let mut n = match stream.read(&mut buffer) {
        Ok(length) => length,
        Err(_) => 0
    };
    if n == 0 {
        return request_as_str;
    }
    request_as_str.push_str(clear_trash_from_buffer(buffer).as_str());
    while n > 0 {
        let mut buffer = vec![0; 5 * 1024 * 1024];
        match stream.read(&mut buffer) {
            Ok(length) => {
                n = length;
            },
            Err(_) => {
                break;
            }
        }
        request_as_str.push_str(clear_trash_from_buffer(buffer).as_str());
    }

    stream.set_nonblocking(false).unwrap();

    request_as_str
}

pub fn extract_request(stream: &mut TcpStream) -> Request {
    let request_as_str: String = get_body(stream);

    let mut additional_body_length: usize = 0;
    let mut read_additional_body: bool = false;
    let re = Regex::new(r"\nExpect: 100-continue\r\n").unwrap();
    if re.is_match(request_as_str.as_str()) {
        read_additional_body = true;
    }

    let mut action: ActionType = ActionType::NULL;
    let mut body = serde_json::Value::Null;
    let mut params: Vec<String> = Vec::new();
    request_as_str.split("\r\n").for_each(|request_row: &str| {
        if request_row.starts_with("POST") || request_row.starts_with("GET") || request_row.starts_with("DELETE") && action == ActionType::NULL {
            (action, params) = extract_action(request_row);
            return; 
        }
        if read_additional_body && request_row.starts_with("Content-Length:") {
            additional_body_length = request_row.split(" ").last().unwrap().to_string().parse::<usize>().unwrap();
            let mut buffer2 = vec![0; additional_body_length];
            stream.read_exact(&mut buffer2).unwrap();
            let json: serde_json::Value = match serde_json::from_slice(buffer2.as_slice()) {
                Ok(json) => json,
                Err(_) => serde_json::Value::Null
            };
            if json != serde_json::Value::Null {
                body = json;
            }
        }
        if !read_additional_body {
            let json: serde_json::Value = match serde_json::from_str(request_row) {
                Ok(json) => json,
                Err(_) => serde_json::Value::Null
            };
            if json != serde_json::Value::Null {
                body = json;
            }
        }
    });
    Request { action, params, body }
}
