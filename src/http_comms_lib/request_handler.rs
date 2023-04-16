use std::{io::Read};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ActionType {
    POP,
    PUSH,
    NULL
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Request {
    pub action: ActionType,
    pub body: serde_json::Value
}

fn extract_request_low_level(buffer: Vec<u8>) -> String {
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

fn extract_action(request_row: &str) -> ActionType {
    let mut action: ActionType = ActionType::NULL;
    request_row.split(" ").for_each(|tiny_slice: &str| {
        if tiny_slice.eq("/pop") && request_row.starts_with("GET") {
            action = ActionType::POP;
        }
        if tiny_slice.eq("/push") && request_row.starts_with("POST") {
            action = ActionType::PUSH;
        }
    });
    action
}

pub fn extract_request(buffer: Vec<u8>) -> Request {
    let http_request = extract_request_low_level(buffer);
    let http_req2 = http_request.clone();
    // println!("{:?}", http_request);
    let mut body = serde_json::Value::Null;
    let mut action: ActionType = ActionType::NULL;
    http_request.split("\r\n").for_each(|request_row: &str| {
        if request_row.starts_with("POST") || request_row.starts_with("GET") && action == ActionType::NULL {
            action = extract_action(request_row);
            return;
        }
        let json: serde_json::Value = match serde_json::from_str(request_row) {
            Ok(json) => json,
            Err(_) => serde_json::Value::Null
        };
        if json != serde_json::Value::Null {
            body = json;
        }
    });
    if body == serde_json::Value::Null && action == ActionType::PUSH {
        println!("{:?}", http_req2);
    } 
    // println!("{:?}", http_req2);
    Request { action, body }
}