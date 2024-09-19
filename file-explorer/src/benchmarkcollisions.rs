use std::fs;
use crate::hashmap::hash_path;

//Benchmark your hashmap collisions
pub fn test_hash_key_collisions(path: String) {
    let mut collision_counter: u64 = 0;
    let mut counter: u64 = 0;
    let mut key_vector: Vec<u64> = Vec::new();
    println!("STARTING COLLISION DETECTION");
    fs_walker(path, &mut key_vector, &mut collision_counter, &mut counter);
    println!("ENDING COLLISION DETECTION");
    let coll_to_counter_ratio: f32 = (collision_counter as f32)/(counter as f32);
    println!("Counter : {}\nCollisions : {}\nCollision to Counter Ratio : {}", counter, collision_counter, coll_to_counter_ratio);
}

fn fs_walker(path: String, key_vec: &mut Vec<u64>, collision_counter: &mut u64, counter: &mut u64) {
    let len_path = path.len() + 1;
    let folder_content = match fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(e) => {
            println!("Got Error for {:?} e {:?}", path, e.to_string());
            return
        }
    };
    for content in folder_content {
        let unwrapped_content: std::path::PathBuf = content.unwrap().path();
        let unwrapped_content_str: &str = unwrapped_content.to_str().unwrap();
        if unwrapped_content.is_dir() {
            fs_walker(unwrapped_content_str.to_string(), key_vec, collision_counter, counter);
        }
        else if unwrapped_content.is_file() {
            *counter = *counter + 1;
            //println!("checking {:?}", unwrapped_content_str);
            let file_name = unwrapped_content.to_str().unwrap()[len_path..].to_string();
            let key = hash_path(&file_name);
            match key_vec.binary_search(&key) {
                Ok(v1) => {
                    //println!("COLLISION DETECTED! for file {} at {}", file_name, unwrapped_content_str);
                    *collision_counter = *collision_counter + 1;
                },
                Err(e) => {
                    key_vec.push(key);
                }
            }
        }
    }
}