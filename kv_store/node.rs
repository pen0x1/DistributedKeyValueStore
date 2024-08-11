use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

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
    }

    fn leave(&mut self) {
    }

    fn handle_task(&self, task: &str) {
        println!("Handling task: {}", task);
    }
}

fn main() {
    dotenv::dotenv().ok();
    let node_address = env::var("NODE_ADDRESS").expect("NODE_ADDRESS must be set");

    let node_id = Uuid::new_v4();

    let node = Node::new(node_id, node_address);

    let _discovered_nodes = node.discover();

    node.handle_task("Example Task");
}