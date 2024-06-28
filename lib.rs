use std::{collections::HashMap, env, io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}};
use std::{thread};

mod kv_store {
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(Clone)]
    pub struct KeyValueStore {
        store: Arc<Mutex<HashMap<String, String>>>,
    }

    impl KeyValueStore {
        pub fn new() -> KeyValueStore {
            KeyValueStore {
                store: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn set(&self, key: &str, value: &str) {
            let mut store = self.store.lock().unwrap();
            store.insert(key.to_string(), value.to_string());
        }

        pub fn get(&self, key: &str) -> Option<String> {
            let store = self.store.lock().unwrap();
            store.get(key).cloned()
        }

        pub fn delete(&self, key: &str) -> Option<String> {
            let mut store = self.store.lock().unwrap();
            store.remove(key)
        }
    }
}

mod server {
    use super::kv_store::KeyValueStore;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::Arc;

    fn handle_client(mut stream: TcpStream, store: Arc<KeyValueStore>) -> std::io::Result<()> {
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                return Ok(());
            }
            let recv = String::from_utf8_lossy(&buffer[..bytes_read]);
            let mut parts = recv.trim().splitn(2, ' ');
            match parts.next() {
                Some("GET") => {
                    if let Some(key) = parts.next() {
                        if let Some(value) = store.get(key) {
                            stream.write_all(value.as_bytes())?;
                        }
                    }
                }
                Some("SET") => {
                    if let Some(key) = parts.next() {
                        if let Some(value) = parts.next() {
                            store.set(key, value);
                        }
                    }
                }
                Some("DELETE") => {
                    if let Some(key) = parts.next() {
                        store.delete(key);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn run_server(address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        let store = Arc::new(KeyValueStore::new());

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let store = Arc::clone(&store);
                    thread::spawn(move || {
                        if let Err(e) = handle_client(stream, store) {
                            eprintln!("Error handling client: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
        Ok(())
    }
}

fn main() {
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:7878".to_string());

    if let Err(e) = server::run_server(&server_address) {
        eprintln!("Failed to start server: {}", e);
    }
}