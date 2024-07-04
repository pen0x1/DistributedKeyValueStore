use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::env;
use serde_json::{self, Value};

struct KeyValueStore {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl KeyValueStore {
    fn new() -> KeyValueStore {
        KeyValueStore {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn handle_client_connection(&self, mut connection: TcpStream) {
        let mut buffer = [0; 1024];
        while match connection.read(&mut buffer) {
            Ok(bytes_read) => {
                let received_data = &buffer[..bytesread];
                if let Ok(request) = serde_json::from_slice::<Value>(received_data) {
                    let response = self.process_request(request);
                    connection.write(response.as_bytes()).unwrap();
                    connection.flush().unwrap();
                }
                true
            }
            Err(_) => false,
        } {}
    }

    fn process_request(&self, request: Value) -> String {
        match request["type"].as_str() {
            Some("set") => {
                let key = request["key"].to_string().trim_matches('"').to_owned();
                let value = request["value"].to_string().trim_matches('"').to_owned();
                let mut data = self.data.lock().unwrap();
                data.insert(key, value);
                "OK\n".to_string()
            },
            Some("get") => {
                let key = request["key"].to_string().trim_matches('"').to_owned();
                let data = self.data.lock().unwrap();
                if let Some(value) = data.get(&key) {
                    format!("{}\n", value)
                } else {
                    "Key not found\n".to_string()
                }
            },
            _ => "Invalid request type\n".to_string(),
        }
    }
}

fn main() {
    let server_address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:7878".to_string());
    let listener = TcpListener::bind(&server_address).expect("Could not bind to address");
    let kv_store = KeyValueStore::new();

    for incoming_connection in listener.incoming() {
        match incoming_connection {
            Ok(connection) => {
                let kv_store_clone = kv_store.clone();
                std::thread::spawn(move || {
                    kv_store_clone.handle_client_connection(connection);
                });
            }
            Err(error) => {
                println!("Failed to establish a connection: {}", error);
            }
        }
    }
}