// Import collections package from STD.
use std::collections::HashMap;

fn main() {
    println!("Learning Rust HashMaps!");

    create_hashmap();
}

pub fn create_hashmap() {
    // Create a new hashmap.
    let mut data = HashMap::new();

    // Insert data into this hashmap.
    data.insert(String::from("One"), 1);
    data.insert(String::from("Seven"), 7);

    // Check the content of the hashmap.
    println!("Content of data: {:?}", data);

    // Accessing value in the HashMap.
    if let Some(data) = data.get("One") {
        println!("The first data here is: {}", data);
    } else {
        // Handling Non-existing Keys
        println!("That data is not found.");
    }

    // Iterating over the hashmap.
    for (key, data) in &data {
        println!("{} matches with {}", key, data);
    }

    // Remove values from the hashmap.
    data.remove("One");
    println!("Content of data: {:?}", data);
}

