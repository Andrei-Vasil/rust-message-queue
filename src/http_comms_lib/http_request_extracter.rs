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

pub fn extract_request(buffer: Vec<u8>, mut stream: TcpStream) -> (Request, TcpStream) {
    let http_request = clear_trash_from_buffer(buffer);

    let mut additional_body_length: usize = 0;
    let mut read_additional_body: bool = false;
    let re = Regex::new(r"\nExpect: 100-continue\r\n").unwrap();
    if re.is_match(http_request.as_str()) {
        read_additional_body = true;
    }

    let mut action: ActionType = ActionType::NULL;
    let mut body = serde_json::Value::Null;
    let mut params: Vec<String> = Vec::new();
    http_request.split("\r\n").for_each(|request_row: &str| {
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
    (Request { action, params, body }, stream)
}
