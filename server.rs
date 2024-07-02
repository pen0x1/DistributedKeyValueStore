use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::env;
use serde_json::{self, Value};

struct DistributedKVStore {
    data_store: Arc<Mutex<HashMap<String, String>>>,
}

impl DistributedKVStore {
    fn new() -> DistributedKVStore {
        DistributedKVStore {
            data_store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn handle_connection(&self, mut connection: TcpStream) {
        let mut buffer = [0; 1024];
        while match connection.read(&mut buffer) {
            Ok(bytes_read) => {
                let received_data = &buffer[..bytes_read];
                if let Ok(request) = serde_json::from_slice::<Value>(received_data) {
                    let response = self.process_client_request(request);
                    connection.write(response.as_bytes()).unwrap();
                    connection.flush().unwrap();
                }
                true
            }
            Err(_) => {
                false
            }
        } {}
    }

    fn process_client_request(&self, request: Value) -> String {
        match request["type"].as_str() {
            Some("set") => {
                let key = request["key"].to_string().trim_matches('"').to_owned();
                let value = request["value"].to_string().trim_matches('"').to_owned();
                let mut data_store = self.data_store.lock().unwrap();
                data_store.insert(key, value);
                "OK\n".to_string()
            },
            Some("get") => {
                let key = request["key"].to_string().trim_matches('"').to_owned();
                let data_store = self.data_store.lock().unwrap();
                if let Some(value) = data_store.get(&key) {
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
    let kv_store = DistributedKVStore::new();

    for incoming_connection in listener.incoming() {
        match incoming_connection {
            Ok(connection) => {
                let kv_store_clone = kv_store.clone();
                std::thread::spawn(move || {
                    kv_store_clone.handle_connection(connection);
                });
            }
            Err(error) => {
                println!("Failed to establish a connection: {}", error);
            }
        }
    }
}