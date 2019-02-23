use std::collections::HashMap;

pub struct MemoryDB {
    pub key_values: HashMap<String, String>
} 

impl MemoryDB {

    pub fn new() -> MemoryDB {
        MemoryDB {
            key_values: HashMap::new(),
        }
    }

    pub fn get_value(&self, key: &str) -> Result<&str, &str> {
        let result = &self.key_values.get(key);
        match result {
            Some(x) => Ok(x),
            None    => Err("Not found!")
        }
    }

    pub fn store_kv(&mut self, k:String, v:String) -> Result<String, &str> {
        let result = self.key_values.insert(k, v);
        match result {
            Some(x) => Ok(x),
            None    => Err("Error!")
        }
    }
}