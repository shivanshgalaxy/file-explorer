use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{collections::HashMap, io::Write};

#[derive(Serialize, Deserialize)]
pub struct cache {
    pub map: HashMap<u64, Vec<String>>,
}

//Write the HashMap to disk || Serialize the HashMap
pub fn cache_the_hash(cache_map: &cache) {
    let json = match serde_json::to_string(&cache_map) {
        Ok(value) => value,
        Err(_e) => panic!("HashMap to JSON Conversion Failed"),
    };
    let mut mycache = match std::fs::File::create("cache.bin") {
        Ok(value) => value,
        Err(_e) => panic!("File Creation Failed"),
    };
    let _ = match mycache.write_all(json.as_bytes()) {
        Ok(v) => v,
        Err(_v) => panic!("Failed to write HashMap JSON to file"),
    };
}

//Read and return the HashMap || Deserialize the HashMap
pub fn get_hash_from_cache() -> HashMap<u64, Vec<String>> {
    let mut mycache = match std::fs::File::open("cache.bin") {
        Ok(value) => value,
        Err(_e) => panic!("File Open Failed"),
    };
    let mut contents = String::new();
    mycache.read_to_string(&mut contents).unwrap();
    let cache: cache = serde_json::from_str(&contents).unwrap();
    cache.map
}
