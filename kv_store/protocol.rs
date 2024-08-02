use serde::{Serialize, Deserialize};
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Put { key: String, value: String },
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
    store: std::collections::HashMap<String, String>,
    node_address: SocketAddr,
}

impl KeyValueStoreProtocol {
    fn new(address: SocketAddr) -> Self {
        KeyValueStoreProtocol {
            store: std::collections::HashMap::new(),
            node_address: address,
        }
    }

    fn handle_message(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Put { key, value } => {
                self.store.insert(key.clone(), value);
                None
            },
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

    let put_msg = Message::Put {
        key: "key1".to_string(),
        value: "value1".to_string(),
    };
    protocol.handle_message(put_msg);

    let get_msg = Message::Get { key: "key1".to_string() };
    if let Some(response) = protocol.handle_message(get_msg) {
        println!("Response: {:?}", response);
    }
}