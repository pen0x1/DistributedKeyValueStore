use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

const STORAGE_FILE_ENV: &str = "KV_STORE_PATH";

#[derive(Serialize, Deserialize)]
struct KvStore {
    store: HashMap<String, String>,
    storage_path: PathBuf,
}

impl KvStore {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let storage_path = env::var(STORAGE_FILE_ENV).map_err(|_| "KV_STORE_PATH must be set and valid")?;
        let path = PathBuf::from_str(&storage_path)?;
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

    fn load_from_file(path: &PathBuf) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let store = serde_json::from_reader(reader)?;
        Ok(store)
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(&self.storage_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.store)?;
        Ok(())
    }

    fn add(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        self.store.insert(key, value);
        self.save_to_file()
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    fn delete(&mut self, key: String) -> Result<(), Box<dyn std::error::Error>> {
        self.store.remove(&key);
        self.save_to_file()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut kv_store = KvStore::new()?;
    kv_store.add("Key1".to_string(), "Value1".to_string())?;
    kv_store.add("Key2".to_string(), "Value2".to_string())?;

    if let Some(value) = kv_store.get("Key1") {
        println!("Value for 'Key1': {}", value);
    }

    kv_store.delete("Key1".to_string())?;
    if kv_store.get("Key1").is_none() {
        println!("Key1 is successfully deleted");
    }

    Ok(())
}