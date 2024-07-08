use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    action: String,
    key: String,
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    result: String,
}

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    let server_address = env::var("KV_STORE_SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());

    let mut stream = TcpStream::connect(server_address)?;

    loop {
        println!("Enter command [GET, SET, DELETE] followed by key and optionally value for SET:");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");

        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        let request = match parts[0].to_uppercase().as_str() {
            "GET" => Request {
                action: "GET".to_string(),
                key: parts[1].to_string(),
                value: None,
            },
            "SET" => Request {
                action: "SET".to_string(),
                key: parts[1].to_string(),
                value: parts.get(2).map(|v| v.to_string()),
            },
            "DELETE" => Request {
                action: "DELETE".to_string(),
                key: parts[1].to_string(),
                value: None,
            },
            _ => {
                println!("Unknown command");
                continue;
            }
        };

        let serialized_request = serde_json::to_string(&request)?;
        stream.write_all(serialized_request.as_bytes())?;

        let mut response = String::new();
        stream.read_to_str