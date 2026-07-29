use std::collections::HashMap;

fn main() {
    let mut vec_1 = vec![45, 30, 85, 90, 41, 49];

    // iter method gives an iterator over immutable references to items in the collection
    // iter_mut gives an iterator over mutable references to the items in the collection
    // into_iter returns an iteraror over own values in the collection
    // let mut vec_1_iter = vec_1.into_iter();
    let mut vec_1_iter = vec_1.iter_mut();
    let value_1 = vec_1_iter.next();
    match value_1 {
        Some(value) => *value = 15,
        None => {}
    }

    // iterate over immutable references
    for values in &vec_1 {
        println!("{values}");
    }

    // iterate over mutable references
    for values in &mut vec_1 {
        println!("{values}");
    }

    // iterate takes ownership of the vector
    for values in vec_1 {
        println!("{values}");
    }

    let mut person: HashMap<String, i32> = HashMap::new();
    person.insert("Hannash".to_string(), 40);
    person.insert("Joseph".to_string(), 44);
    person.insert("Sara".to_string(), 55);

    // key (name) borrows as immutable
    for (name, age) in &mut person {
        println!("The person {} has an age of {}", name, age);
    }

    // take the ownership
    for (name, age) in person {
        println!("The person {} has an age of {}", name, age);
    }
}
