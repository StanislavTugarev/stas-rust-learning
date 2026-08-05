use std::collections::HashMap;

fn main() {
    let text = "one two two three three three";
    let map = word_count(text);
    let top_word = map.iter().max_by_key(|x| x.1);
    if let Some(top_word) = top_word{
        println!("top word is {} with value: {}", top_word.0, top_word.1)
    }
}

fn word_count(text: &str) -> HashMap<String, u32> {
    let mut mapping = HashMap::new();
    for word in text.split_whitespace(){
        *mapping.entry(word.to_string().to_lowercase()).or_insert(0) += 1
    };
    mapping
}