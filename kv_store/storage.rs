use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

const STORAGE_FILE_ENV: &str = "KV_STORE_PATH";

struct KvStore {
    store: HashMap<String, String>,
    storage_path: Path. Buf,
}

impl KvStore {
    fn new() -> io::Result<Self> {
        let storage_path = env::var(STORAGE_FILE_ENV).unwrap_or_else(|_| "kv_store.db".to_string());
        let path = PathBuf::from(storage_path.clone());
        let store = if path.exists() {
            KvStore::load_from_file(&path)?
        } else {
            HashMap::new()
        };
        Ok(KvStore {
            store,
            storage_path: path,
        })
    }

    fn load_from_file(path: &PathBuf) -> io::Result<HashMap<String, String>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let store = serde_json::from_str(&contents).unwrap_or_else(|_| HashMap::new());
        Ok(store)
    }

    fn save_to_file(&self) -> io::Result<()> {
        let mut file = File::create(&self.storage_path)?;
        let contents = serde_json::to_string(&self.store).expect("Serialization failed");
        file.write_all(contents.as_bytes())
    }

    fn add(&mut self, key: String, value: String) -> io::Result<()> {
        self.store.insert(key, value);
        self.save_to_file()
    }

    fn get(&self, key: String) -> Option<&String> {
        self.store.get(&key)
    }

    fn delete(&mut self, key: String) -> io::Result<()> {
        self.store.remove(&key);
        self.save_to_file()
    }
}

fn main() {
    let mut kv_store = KvStore::new().expect("Failed to initiate KvStore");
    kv_store.add("Key1".to_string(), "Value1".to_string()).expect("Failed to add key-value");
    kv_store.add("Key2".to_string(), "Value2".to_string()).expect("Failed to add key-value");

    if let Some(value) = kv_store.get("Key1".to_string()) {
        println!("Value for 'Key1': {}", value);
    }

    kv_store.delete("Key1".to_string()).expect("Failed to delete key-value");
    if kv_store.get("Key1".to_string()).is_none() {
        println!("Key1 is successfully deleted");
    }
}