// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::time::Instant;

mod walker;
mod hashmap;
mod cachehash;

#[tauri::command]
fn query_hashmap(name: String) -> Vec<String> {
    let mut fs_hash_map: HashMap<u64, Vec<String>> = HashMap::new();
    let init_path = "/home/".to_string();

    //Make new cache.bin if it doesn't exist
    if !(std::path::Path::new("cache.bin").exists()) {
        println!("cache.bin not found\nRemaking cache.bin");
        let nowa: Instant = Instant::now();
        hashmap::hash_map_of_target_location(&mut fs_hash_map, init_path.to_string());
        let cache = cachehash::cache { map: fs_hash_map };
        cachehash::cache_the_hash(&cache);
        let nowb: std::time::Duration = nowa.elapsed();
        println!("Remaking cache.bin took {:?}", nowb);
    }

    // 2.34% collision percentage benchmarked on C drive
    // hashmap::test_hash_key_collisions(formatted_path);

    //Load the HashMap from cache.bin
    println!("Loading cache.bin into memory");
    let now_read_hash_from_cache: Instant = Instant::now();
    fs_hash_map = cachehash::get_hash_from_cache();
    let time_taken_to_read_hash_from_cache: std::time::Duration = now_read_hash_from_cache.elapsed();

    println!("Reading HashMap from cache.bin took {:?}", time_taken_to_read_hash_from_cache);

    match fs_hash_map.get(&hashmap::hash_path(&name)) {
        Some(value) => {
            return value.to_vec();
        },
        None => {
            return vec!["Given File Not Found".to_string()];
        }
    
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            walker::get_files_and_details_as_vector,
            query_hashmap
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
