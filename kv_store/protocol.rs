use serde::{Serialize, Deserialize};
use std::env;
use std::net::SocketAddr;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Put { key: String, value: String },
    BatchPut(Vec<(String, String)>),
    Fetch { key: String },
    Reply { key: Option<String>, value: Option<String> },
}

impl Command {
    // Utilize serde for direct serialization/deserialization without explicit method calls
    // Methods are now integrated into Enum usage, see process_command for deserialization example
}

struct KeyValueStore {
    data: HashMap<String, String>,
    address: SocketAddr,
}

impl KeyValueStore {
    fn initialize(address: SocketAddr) -> Self {
        KeyValueStore {
            data: HashMap::new(),
            address,
        }
    }

    fn process_command(&mut self, command: Command) -> Option<Command> {
        match command {
            Command::Put { key, value } => {
                self.data.insert(key, value);
                None 
            },
            Command::BatchPut(pairs) => {
                for (key, value) in pairs {
                    self.data.insert(key, value);
                }
                None 
            }
            Command::Fetch { key } => {
                self.data.get(&key).cloned().map(|value| Command::Reply { key: Some(key), value: Some(value) })
            }
            // Removed the unreachable pattern to keep the function concise
        }
    }
}

fn main() {
    // Simplified environment variable handling and parsing
    let addr_str = env::var("NODE_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let socket_address: SocketAddr = addr_str.parse().expect("Invalid NODE_ADDRESS format");

    let mut store = KeyValueStore::initialize(socket_address);

    // Direct declaration and processing of commands for clarity
    store.process_command(Command::BatchPut(vec![
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]));

    if let Some(reply) = store.process_command(Command::Fetch { key: "key1".to_string() }) {
        println!("Reply: {:?}", reply);
    }
}