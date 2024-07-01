use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::env;
use serde_json::{self, Value};

struct KeyValueStore {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl KeyValueStore {
    fn new() -> KeyValueStore {
        KeyValueStore {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        while match stream.read(&mut buffer) {
            Ok(size) => {
                let data = &buffer[..size];
                if let Ok(request) = serde_json::from_slice::<Value>(data) {
                    let response = self.process_request(request);
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                true
            }
            Err(_) => {
                false
            }
        } {}
    }

    fn process_request(&self, request: Value) -> String {
        if request["type"] == "set" {
            let key = request["key"].to_string();
            let value = request["value"].to_string();
            let mut map = self.store.lock().unwrap();
            map.insert(key, value);
            "OK\n".to_string()
        } else if request["type"] == "get" {
            let key = request["key"].to_string();
            let map = self.store.lock().unwrap();
            if let Some(value) = map.get(&key) {
                format!("{}\n", value)
            } else {
                "Key not found\n".to_string()
            }
        } else {
            "Invalid request type\n".to_string()
        }
    }
}

fn main() {
    let address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:7878".to_string());
    let listener = TcpListener::bind(&address).expect("Could not bind");
    let store = KeyValueStore::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = store.clone();
                std::thread::spawn(move || {
                    store.handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}