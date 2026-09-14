use std::num::ParseIntError;

// Option<Result<T, E>>
// Three-way Outcome -> None -> None                                Operation not attempted
//                   -> Some(value) -> Some(Ok(valid_age))          Operation attempted and successful
//                   -> Some(age)   -> Some(Err(ParseIntError))     Operation attempted and failed

fn handle_user_registration(
    name: &str,
    age_input: Option<&str>,
) -> Option<Result<u32, ParseIntError>> {
    println!("Registering user: {}", name);
    age_input.map(|s| s.parse::<u32>())
}

fn main() {
    match handle_user_registration("Alice", Some("25")) {
        None => println!("Age not provided, continuing without age"),
        Some(Ok(age)) => println!("User age is valid: {}", age),
        Some(Err(e)) => println!("Invalid age input: {}", e),
    }
}
