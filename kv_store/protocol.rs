use serde::{Serialize, Deserialize};
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Put { key: String, value: String },
    BatchPut(Vec<(String, String)>),
    Fetch { key: String },
    Reply { key: Option<String>, value: Option<String> },
}

impl Command {
    fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn deserialize(bytes: &[u8]) -> Self {
        serde_json::from_slice(bytes).unwrap()
    }
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
                let response = self.data.get(&key).cloned().map(|value| {
                    Command::Reply { key: Some(key), value: Some(value) }
                });
                response
            }
            _ => None,
        }
    }
}

fn main() {
    let address_env_var = env::var("NODE_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let socket_address = SocketAddr::from_str(&address_env_var).expect("Invalid NODE_ADDRESS format");

    let mut store = KeyValueStore::initialize(socket_address);

    let entries_for_batch_put = vec![
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string())
    ];
    let batch_put_command = Command::BatchPut(entries_for_batch_put);
    store.process_command(batch_put_command);

    let fetch_command = Command::Fetch { key: "key1".to_string() };
    if let Some(reply) = store.process_command(fetch_command) {
        println!("Reply: {:?}", reply);
    }
}