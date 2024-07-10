use std::collections::HashMap;
use std::env;
use std::io::{self, Write, Read};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct KeyValueRequest {
    action: String,
    key: String,
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct KeyValueResponse {
    result: String,
}

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    let server_address = env::var("KV_STORE_SERVER_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let mut tcp_connection = TcpStream::connect(server_address)
        .expect("Failed to connect to server");

    loop {
        println!("Enter command [GET, SET, DELETE] followed by key and optionally value for SET:");
        let mut user_input = String::new();
        if io::stdin().read_line(&mut user_input).is_err() {
            eprintln!("Error reading input. Please try again.");
            continue;
        }

        let input_tokens: Vec<&str> = user_input.trim().split_whitespace().collect();
        if input_tokens.len() < 2 {
            eprintln!("Invalid command format. Please specify at least a command and a key.");
            continue;
        }

        let kv_request = match input_tokens[0].to_uppercase().as_str() {
            "GET" | "DELETE" if input_tokens.len() >= 2 => KeyValueRequest {
                action: input_tokens[0].to_uppercase(),
                key: input_tokens[1].to_string(),
                value: None,
            },
            "SET" if input_tokens.len() >= 3 => KeyValueRequest {
                action: "SET".to_string(),
                key: input_tokens[1].to_string(),
                value: input_tokens.get(2).map(|v| v.to_string()),
            },
            _ => {
                println!("Unknown command or incorrect number of arguments.");
                continue;
            }
        };

        let serialized_kv_request = match serde_json::to_string(&kv_request) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to serialize request: {}", e);
                continue;
            }
        };

        if let Err(e) = tcp_connection.write_all(serialized_kv_request.as_bytes()) {
            eprintln!("Failed to send data to server: {}", e);
            continue;
        }

        let mut response_buffer = Vec::new();
        match tcp_connection.read_to_end(&mut response_buffer) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to read response from server: {}", e);
                continue;
            }
        };

        let server_response = match String::from_utf8(response_buffer) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to decode server response: {}", e);
                continue;
            }
        };

        println!("Response: {}", server_json::Response);
    }
}