use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

mod kv_store {
    use super::*;

    pub struct KeyValueStore {
        store: HashMap<String, String>,
    }

    impl KeyValueStore {
        pub fn new() -> KeyValueStore {
            KeyValueStore {
                store: HashMap::new(),
            }
        }

        pub fn set(&mut self, key: &str, value: &str) {
            self.store.insert(key.to_string(), value.to_string());
        }

        pub fn get(&self, key: &str) -> Option<&String> {
            self.store.get(key)
        }

        pub fn delete(&mut self, key: &str) -> Option<String> {
            self.store.remove(key)
        }
    }
}

mod server {
    use super::*;

    pub fn run_server(address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
        let mut buffer = [0; 1024];
        while let Ok(size) = stream.read(&mut buffer)? {
            if size == 0 {
                break; // End of stream
            }
            stream.write_all(&buffer[..size])?;
        }
        Ok(())
    }
}

mod client {
    use super::*;

    pub fn connect_to_ server(address: &str) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(address)?;

        let msg = "Hello from the client!";
        stream.write_all(msg.as_bytes())?;

        let mut buffer = [0; 1024];
        let _ = stream.read(&mut buffer)?; // Now checking the result

        // Potentially use received data

        Ok(())
    }
}

fn get_server_address() -> String {
    env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:7878".to_string())
}

pub fn run() {
    let server_address = get_server_address();

    if let Err(e) = server::run_server(&server_address) {
        eprintln!("Failed to start server: {}", e);
    };
}