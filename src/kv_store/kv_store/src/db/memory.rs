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

    pub fn get_key(&self, s: String) -> String {
        let result = &self.key_values.get(&s);
        match result {
            Some(x) => format!("{}", x),
            None    => return String::from("key not found!") // throw err here
        }

    }

    pub fn store_kv(&mut self, k:String, v:String) {
        self.key_values.insert(k, v);
    }
}