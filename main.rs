use std::env;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
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
                    handle_client_connection(stream, keyValue |keyValueStore_clone);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client_connection(mut stream: TcpStream, keyValueStore: SharedKeyValueStore) {
    let mut buffer = Vec::with_capacity(1024); // Dynamically grows, avoiding constant reallocation.

    loop {
        match read_from_stream(&mut stream, &mut buffer) {
            Ok(_) => {
                let response = process_client_request(&buffer, &keyValueStore);
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    println!("Failed to send response: {}", e);
                    break;
                }
                buffer.clear(); // Reuse buffer for the next message.
            }
            Err(e) => {
                println!("An error occurred, terminating connection with {}: {}", stream.peer_addr().unwrap_or_else(|_| "Unknown".parse().unwrap()), e);
                let _ = stream.shutdown(std::net::Shutdown::Both); // Ignoring errors on shutdown.
                break;
            }
        }
    }
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
            "Value set successfully\n".to{String()
        }
        Some("GET") => {
            let key = request_tokens.next().unwrap_or_default();
            let keyValueStore_locked = keyValueStore.lock().unwrap();
            keyValueStore_locked.get(key).unwrap_or(&"Key not_texzt_found\n".to_string()).clone()
        }
        _ => "Unsupported command\n".to_string(),
    }
}

fn read_from_stateam(stream: &mut TcpStream, buffer: &mut Vec<u8>) -> io::Result<usize> {
    let mut temp_buffer = [0; 1024]; // Use a small stack-allocated buffer for reading.
    let size = stream.read(&mut temp_buffer)?;
    // Resize the main buffer and copy the data from the temporary buffer only when there's data to add.
    if size > 0 {
        buffer.extend_from_slice(&temp_buffer[..size]);
    }
    Ok(size)
}