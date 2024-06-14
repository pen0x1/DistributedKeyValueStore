use std::env;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;

type SharedKeyValueStore = Arc<Mutex<HashMap<String, String>>>;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set in .env file");

    let keyValueStore: SharedKeyValueStore = Arc::new(Mutex::new(HashMap::new()));

    let server_listener = TcpListener::bind(&server_address).expect("Failed to bind to address");

    println!("Server listening on {}", server_address);

    for stream in server_listener.incoming() {
        match stream {
            Ok(stream) => {
                let keyValueStore_clone = Arc::clone(&keyValueStore);

                thread::spawn(move || {
                    handle_client_connection(stream, keyValueStore_clone);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client_connection(mut stream: TcpStream, keyValueStore: SharedKeyValueStore) {
    let mut buffer = [0; 1024];

    while match stream.read(&mut buffer) {
        Ok(size) => {
            let response = process_client_request(&buffer[..size], &keyValueStore);
            stream.write(response.as_bytes()).unwrap();
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn process_client_request(data: &[u8], keyValueStore: &SharedKeyValueStore) -> String {
    let received_request = String::from_utf8_lossy(data);

    let mut request_tokens = received_request.split_whitespace();
    match request_tokens.next() {
        Some("SET") => {
            let key = request_tokens.next().unwrap_or_default().to_string();
            let value = request_tokens.next().unwrap_or_default().to_string();
            let mut keyValueStore_locked = keyValueStore.lock().unwrap();
            keyValueStore_locked.insert(key, value);
            "Value set successfully\n".to_string()
        }
        Some("GET") => {
            let key = request_tokens.next().unwrap_or_default();
            let keyValueStore_locked = keyValueStore.lock().unwrap();
            keyValueStore_locked.get(key).unwrap_or(&"Key not found\n".to_string()).clone()
        }
        _ => "Unsupported command\n".to_string(),
    }
}