use std::collections::HashMap;

fn main() {
    let mut list: HashMap<String, i32> = HashMap::new();
    
    list.insert("one".to_string(), 1);
    list.insert("two".to_string(), 2);
    list.insert("three".to_string(), 8);
    list.insert("six".to_string(), 8);

    println!("{:?}", list);

    // Iterating over the HashMap
    for (k, v) in &list {
        println!("{} - {}", k, v);
    }

    println!("print");

    // Check if the key "six" exists
    if list.contains_key("six") {
        println!("Key exists: six - {:?}", list.get("six"));
    } else {
        list.insert("ten".to_string(), 10);
    }

    // Creating a new HashMap with initial values using 'collect'
    let hashlist: HashMap<&str, f64> = [("Mercury", 0.5), ("Venus", 0.8), ("Tuy", 0.9), ("Toy", 0.7)]
        .iter()
        .cloned()
        .collect();

    println!("hashlist: {:?}", hashlist);
}
