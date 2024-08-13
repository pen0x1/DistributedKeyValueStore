use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::net::{TcpListener, TcpStream, SocketAddr};

struct Node {
    id: Uuid,
    address: String,
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Node {
    fn new(id: Uuid, address: String) -> Self {
        Node {
            id,
            address,
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn discover(&self) -> Vec<Node> {
        vec![]
    }

    fn join(&mut self, other_node: &Node) {
        println!("Joining node with address: {}", &other_node.address);
    }

    fn leave(&mut self) {
    }

    fn handle_task(&self, task: &str) {
        println!("Handling task: {}", task);
    }
  
    fn set(&self, key: String, value: String) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    fn start_server(&self) {
        let listener = TcpListener::bind(&self.address).expect("Could not bind to address");
        println!("Node server running on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(e) => {
                    println!("Failed to handle incoming connection: {}", e);
                }
            }
        }
    }

    fn handle_connection(&self, _stream: TcpStream) {
        println!("Got connection from a node!");
    }
}

fn main() {
    dotenv::dotenv().ok();
    let node_address = env::var("NODE_ADDRESS").expect("NODE_ADDRESS must be set");
    
    let node_id = Uuid::new_v4();
    let node = Node::new(node_id, node_address.clone());

    let _discovered_nodes = node.discover();
    node.handle_task("Example Task");

    node.set("key1".to_string(), "value1".to_string());
    if let Some(value) = node.get("key1") {
        println!("Retrieved value: {}", value);
    }
}