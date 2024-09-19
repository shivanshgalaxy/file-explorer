use std::collections::HashMap;
use std::time::Instant;
use std::env;

mod hashmap;
mod cachehash;
mod benchmarkcollisions;

fn main() {
    //Parse argument for file name
    let search_string = match env::args().nth(1) {
        Some(value) => value,
        None => panic!("No Argument Given")
    };

    let init_path = "/home/";
    let mut fs_hash_map: HashMap<u64, Vec<String>> = HashMap::new();

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

    //Search for the file
    println!("Searching for {}", search_string);
    let now1: Instant = Instant::now();
    hashmap::hash_map_get_path(&fs_hash_map, hashmap::hash_path(&search_string));
    let now2: std::time::Duration = now1.elapsed();

    println!("Took {:?} to Search for {}", now2, search_string);
}

