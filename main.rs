use std::env;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;

type KeyValueStore = Arc<Mutex<HashMap<String, String>>>;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set in .env file");

    let store: KeyValueStore = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind(&address).expect("Failed to bind to address");

    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);

                thread::spawn(move || {
                    handle_client(stream, store);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, store: KeyValueStore) {
    let mut buffer = [0; 1024];

    while match stream.read(&mut buffer) {
        Ok(size) => {
            let response = process_data(&buffer[..size], &store);
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

fn process_data(data: &[u8], store: &KeyValueStore) -> String {
    let received = String::from_utf8_lossy(data);

    let mut commands = received.split_whitespace();
    match commands.next() {
        Some("SET") => {
            let key = commands.next().unwrap_or_default().to_string();
            let value = commands.next().unwrap_or_default().to_string();
            let mut store = store.lock().unwrap();
            store.insert(key, value);
            "Value set successfully\n".to_string()
        }
        Some("GET") => {
            let key = commands.next().unwrap_or_default();
            let store = store.lock().unwrap();
            store.get(key).unwrap_or(&"Key not found\n".to_string()).clone()
        }
        _ => "Unsupported command\n".to_string(),
    }
}