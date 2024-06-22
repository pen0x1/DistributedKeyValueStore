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

        pub fn set(&mut self, key: String, value: String) {
            self.store.insert(key, value);
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
                    handle_client(stream);
                }
                Err(e) => { }
            }
        }
        Ok(())
    }

    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        while match stream.read(&mut buffer) {
            Ok(size) => {
                stream.write_all(&buffer[..size]).unwrap();
                true
            }
            Err(_) => {
                false
            }
        } {}
    }
}

mod client {
    use super::*;

    pub fn connect_to_server(address: &str) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(address)?;

        let msg = "Hello from the client!";
        stream.write_all(msg.as_bytes())?;
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        Ok(())
    }
}

fn get_server_address() -> String {
    env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:7878".to_string())
}

pub fn run() {
    let server_address = get_server_address();

    match server::run_server(&server_address) {
        Ok(()) => {},
        Err(e) => {},
    };
}