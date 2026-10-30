// Coercion: One_type -> Another_type (implicitly)

// pub trait Deref {
// type Target: ?Sized;
//
// fn deref(&self) -> &Self::Target;
//}

// Deref Coercion: &T -> &U
// https://doc.rust-lang.org/std/ops/trait.Deref.html
// https://doc.rust-lang.org/std/ops/trait.DerefMut.html

use std::ops::Deref;

struct HtmlText {
    contents: String,
    is_escaped: bool,
}

impl Deref for HtmlText {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let s = "Some string".to_string();
    accept_str(&s);

    let html_input = HtmlText {
        contents: "<b>Hello</b>".to_string(),
        is_escaped: false,
    };

    // Coercion: &HtmlText -> &String
    print_length(&html_input); // print_length(&html_input.deref())
    if html_input.contains("Hello") {
        println!("Found 'Hello' in HTML content!")
    };

    // Common Deref Coercion
    // 1. &String -> &str
    // 2. &Vec<T> -> &[T]
    // 3. &Box<T> -> &T

    let array: &[i32];
    let vec = &vec![1, 2, 3];
    array = vec;

    let i: &i32;
    let value = Box::new(5);
    i = &value;
}

fn accept_str(s: &str) {
    println!("Received: {s}");
}
