use core::num;
use std::num::ParseIntError;

fn main() {}

fn read_number(input: &str) -> Result<i32, ParseIntError> {
    let num = match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };

    let num = input.trim().parse::<i32>()?; // returns error
    Ok(num)
}

fn extract_username(email: &str) -> Option<&str> {
    let at_pos = email.find('@')?; // returns None if there's no "@"
    let username = email.get(0..at_pos)?; // if returns Some variant, we continue
    Some(username)
}
