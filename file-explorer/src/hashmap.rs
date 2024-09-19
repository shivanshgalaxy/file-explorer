use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher}; 
use std::fs;

//Hash Function
pub fn hash_path(path: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    hasher.finish()
}

//Create a HashMap of target location Recursively
pub fn hash_map_of_target_location(hashmap: &mut HashMap<u64, Vec<String>>, path: String) {
    let folder_content = match fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(_e) => {
            // println!("Got Error for {:?} e {:?}", path, _e);
            return
        }
    };
    let len_path = path.len() + 1;
    for content in folder_content {
        let unwrapped_content = content.unwrap().path();
        let unwrapped_content_str = unwrapped_content.to_str().unwrap().to_string();
        if unwrapped_content.is_dir() {
            hash_map_of_target_location(hashmap, unwrapped_content_str);
        }
        else {
            let file_name = unwrapped_content.to_str().unwrap()[len_path..].to_string();
            match hashmap.entry(hash_path(&file_name)) {
                // If the key exists, push the value to the vector
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push(unwrapped_content_str);
                }
                // If the key does not exist, insert a new vector with the value
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(vec![unwrapped_content_str]);
                }
            }
        }
    }
}

//Search for a key in the HashMap
pub fn hash_map_get_path(hashmap: &HashMap<u64, Vec<String>>, key: u64) {
    match hashmap.get(&key) {
        Some(value) => {
            for i in value {
                println!("File Found at {}", *i);
            }
        },
        None => println!("Given File Not Found")
    };
}