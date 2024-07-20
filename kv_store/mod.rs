pub mod kv_store {
    pub mod storage {
        use std::collections::HashMap;
        use std::env;

        pub struct Storage {
            data: HashMap<String, String>,
            cache: HashMap<String, String>, // Simple cache to avoid redundant lookups
        }

        impl Storage {
            pub fn new() -> Self {
                let mut storage = Storage {
                    data: HashMap::new(),
                    cache: HashMap::new(),
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
                self.data.insert(key.clone(), value.clone());
                self.cache.insert(key, value); // Update cache as well when setting a new value
            }

            pub fn get(&self, key: &str) -> Option<&String> {
                // Attempt to retrieve from cache first
                if let Some(value) = self.cache.get(key) {
                    return Some(value);
                }
                // Fallback to data lookup and cache the result
                if let Some(value) = self.data.get(key) {
                    self.cache.insert(key.to_string(), value.clone()); // Cache the result, requires making cache mutable
                    return Some(value);
                }
                // If key is not present in both, return None
                None
            }
        }
    }

    pub mod protocol {
        pub struct ProtocolHandler;

        impl ProtocolPowerShellHandler {
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