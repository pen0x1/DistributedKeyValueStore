pub mod kv_store {
    pub mod storage;
    pub mod protocol;
    pub mod node;

    pub mod storage {
        use std::collections::HashMap;
        use std::env;

        pub struct Storage {
            data: HashMap<String, String>,
        }

        impl Storage {
            pub fn new() -> Self {
                let mut storage = Storage {
                    data: HashMap::new(),
                };
                if let Ok(env_data) = env::var("STORAGE_INITIAL_DATA") {
                    for entry in env_data.split(',') {
                        let mut parts = entry.split('=');
                        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                            storage.set(key.to_string(), value.to_string());
                        }
                    }
                }

                storage
            }

            pub fn set(&mut self, key: String, value: String) {
                self.data.insert(key, value);
            }

            pub fn get(&self, key: &str) -> Option<&String> {
                self.data.get(key)
            }
        }
    }

    pub mod protocol {
        pub struct ProtocolHandler;

        impl ProtocolHandler {
            pub fn new() -> Self {
                ProtocolHandler {}
            }

            pub fn handle(&self, _data: &str) -> bool {
                true
            }
        }
    }

    pub mod node {
        pub struct Node;

        impl Node {
            pub fn new() -> Self {
                Node {}
            }

            pub fn operate(&self) -> bool {
                true
            }
        }
    }
}