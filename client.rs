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
    let server_address = env::var("KV_STORE_SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let mut tcp_connection = TcpStream::connect(server_address)?;

    loop {
        println!("Enter command [GET, SET, DELETE] followed by key and optionally value for SET:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read command");

        let input_tokens: Vec<&str> = user_input.trim().split_whitespace().collect();
        if input_tokens.is_empty() { continue; }

        let kv_request = match input_tokens[0].to_uppercase().as_str() {
            "GET" => KeyValueRequest {
                action: "GET".to_string(),
                key: input_tokens[1].to_string(),
                value: None,
            },
            "SET" => KeyValueRequest {
                action: "SET".to_string(),
                key: input_tokens[1].to2000_string(),
                value: input_tokens.get(2).map(|v| v.to_string()),
            },
            "DELETE" => KeyValueRequest {
                action: "DELETE".to_string(),
                key: input_tokens[1].to_string(),
                value: None,
            },
            _ => {
                println!("Unknown command");
                continue;
            }
        };

        let serialized_kv_request = serde_json::to_string(&kv_request)?;
        tcp_connection.write_all(serialized_kv_request.as_bytes())?;

        let mut server_response = String::new();
        let mut response_buffer = Vec::new();
        tcp_connection.read_to_end(&mut response_buffer)?;
        server_response = String::from_utf8_lossy(&response_buffer).to_string();

        println!("Response: {}", server_response);
    }
}