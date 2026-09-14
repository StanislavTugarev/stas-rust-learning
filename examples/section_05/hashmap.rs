use std::{collections::HashMap, hash::Hash};
fn main() {
    // Btree map?
    // index map?
    let worlds = vec!["Hello", "World", "Rust", "Programming"];
    let counts = vec![5, 2, 15, 5];

    let word_counts = vec![
        ("Hello", 5),
        ("Worlds", 2),
        ("Rust", 15),
        ("Programming", 5),
    ];
    let target_words = word_counts.contains(&("Hello", 5));

    // in a hashmap all keys are unique and inserting a value with existing key will rewrite it
    let mut word_counts: HashMap<&str, u8> = HashMap::new();
    word_counts.insert("Hello", 5);
    word_counts.insert("World", 2);
    word_counts.insert("Rust", 15);
    word_counts.insert("Programming", 5);

    let has_programming_key = word_counts.contains_key("Programming");
    let programming_count = word_counts.get("Programming");

    // Check if key exists in hashmap, if not - insert it
    let new_entry = word_counts.entry("C++").or_insert(0);
    println!("HashMap: {:?}", word_counts);
}
