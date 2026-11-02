// least upper bound coercion occurs:
// a series of if/else branches
// a series of match arms
// a series of array elements

use std::{any::Any, ops::Deref};

struct JsonValue {
    raw: String,
    is_parsed: bool,
}
impl Deref for JsonValue {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

struct A;

struct B;

struct C;

fn main() {
    let condition = true;
    let result: &str = if condition {
        &"Hello".to_string() //&String -> &str
    } else {
        // &JsonValue -> &String -> &str
        &JsonValue {
            raw: r#"{"name": "Alice", "age": 30}"#.to_string(),
            is_parsed: false,
        }
    };

    let value = 3;
    let a = &A;
    let b = &B;
    let c = &C;

    let result: &dyn Any = match value {
        1 => a,
        2 => b,
        _ => c,
    };

    let string_array = [
        "Rust",
        &"Rust".to_string(),
        &JsonValue {
            raw: r#"{"name": "Alice", "age": 30}"#.to_string(),
            is_parsed: false,
        },
    ];
}
