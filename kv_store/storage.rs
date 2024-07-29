use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

const STORAGE_FILE_PATH_ENV_VAR: &str = "KV_STORE_PATH";

#[derive(Serialize, Deserialize)]
struct KeyValueStore {
    data: HashMap<String, String>,
    file_path: PathBuf,
}

impl KeyValueStore {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let storage_path_str = env::var(STORAGE_FILE_PATH_ENV_VAR)
            .map_err(|_| "Environment variable KV_STORE_PATH must be set and valid")?;
        let file_path = PathBuf::from_str(&storage_path_str)?;
        let data = if file_path.exists() {
            KeyValueStore::load_from_file(&file_path)?
        } else {
            HashMap::new()
        };
        Ok(KeyValueStore {
            data,
            file_path,
        })
    }

    fn load_from_file(file_path: &PathBuf) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        Ok(data)
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(&self.file_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.data)?;
        Ok(())
    }

    fn add(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        self.data.insert(key, value);
        self.save_to_file()
    }

    fn retrieve(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn remove(&mut self, key: String) -> Result<(), Box<dyn std::error::Error>> {
        self.data.remove(&key);
        self.save_to_file()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut kv_store = KeyValueStore::new()?;
    kv_store.add("Key1".to_string(), "Value1".to_string())?;
    kv_store.add("Key2".to_string(), "Value2".to_string())?;

    if let Some(value) = kv_store.retrieve("Key1") {
        println!("Value for 'Key1': {}", value);
    }

    kv_store.remove("Key1".to_string())?;
    if kv_store.retrieve("Key1").is_none() {
        println!("Key1 is successfully removed");
    }

    Ok(())
}