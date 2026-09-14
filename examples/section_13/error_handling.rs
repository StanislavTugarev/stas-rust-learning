use core::panic;
use std::fs::File;

// Unrecoverable errors
// Unimplemented code
fn unimplemented_feature() {
    panic!("this feature is not implemented yet");
}

// Invalid values
fn value_processing(value: i32) {
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => panic!("unexpected value: {}", value),
    }
}

// Critical conditions of tests
fn must_be_positive(n: i32) {
    assert!(n > 0, "Value must be positive, got {}", n);
}

// Index out of bound
fn out_of_bound() {
    let v = vec![1, 2, 3];
    println!("{}", v[5]);
}

// Recoverable errors
fn main() {
    // File opening
    let file = File::open("missing.txt");
    match file {
        Ok(f) => println!("File opened successfully: {:?}", f),
        Err(e) => println!("Failed to open file: {}", e),
    }

    // Parsing integer
    let user_input = "42a";
    match parse_input_to_int(user_input) {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Invalid input: {}", e),
    }
}

fn parse_input_to_int(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.trim().parse::<i32>()
}
