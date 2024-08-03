use serde::{Serialize, Deserialize};
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Put { key: String, value: String },
    BatchPut(Vec<(String, String)>), // New variant for batching put requests
    Get { key: String },
    Response { key: Option<String>, value: Option<String> },
}

impl Message {
    fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn deserialize(bytes: &[u8]) -> Self {
        serde_json::from_slice(bytes).unwrap()
    }
}

struct KeyValueStoreProtocol {
    store: HashMap<String, String>,
    node_address: SocketAddr,
}

impl KeyValueStoreProtocol {
    fn new(address: SocketAddr) -> Self {
        KeyValueStoreProtocol {
            store: HashMap::new(),
            node_address: address,
        }
    }

    fn handle_message(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Put { key, value } => {
                self.store.insert(key, value);
                None
            },
            Message::BatchPut(pairs) => {
                for (key, value) in pairs {
                    self.store.insert(key, value);
                }
                None
            }
            Message::Get { key } => {
                let response = self.store.get(&key).cloned().map(|value| {
                    Message::Response { key: Some(key), value: Some(value) }
                });
                response
            }
            _ => None,
        }
    }
}

fn main() {
    let node_address = env::var("NODE_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let addr = SocketAddr::from_str(&node_address).expect("Failed to parse NODE_ADDRESS");

    let mut protocol = KeyValueStoreProtocol::new(addr);

    // Example of using BatchPut
    let pairs_to_put = vec![
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string())
    ];
    let batch_put_msg = Message::BatchPut(pairs_to_put);
    protocol.handle_message(batch_put_msg);

    // Getting a value to demonstrate it's been put correctly
    let get_msg1 = Message::Get { key: "key1".to_string() };
    if let Some(response) = protocol.handle_message(get_msg1) {
        println!("Response: {:?}", response);
    }

    // Repeat for key2 or any key as needed
}