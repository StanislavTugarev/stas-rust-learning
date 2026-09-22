use std::panic::set_hook;

fn main() {
    // for running with stack backtrace
    // unsafe {
    //     std::env::set_var("RUST_BACKTRACE", "1");
    // };
    // handle_request();

    set_hook(Box::new(|info| {
        eprintln!("Custom Panic: {}", info);
        if let Some(location) = info.location() {
            eprintln!(
                "Occurred in file {} at line {}",
                location.file(),
                location.line()
            );
        }
    }));

    // panic!("Main function panicked");

    let my_string = "my super string".to_string();
    set_hook(Box::new(|info| {
        let message = info.payload().downcast_ref::<&str>();

        // println!("{}", my_string);
        match message {
            Some(&"Database error") => println!("Panicked due to DB error"),
            Some(&"Config missing") => println!("Panicked due to missing configuration info"),
            Some(msg) => println!("General panic: {}", msg),
            None => println!("Unknown panic"),
        }
    }));
    panic!("Some Error");
}

fn handle_request() {
    println!("Handling login request ...");
    process_login("Alice", "");
}

fn process_login(username: &str, password: &str) {
    println!("Processing login for user: {}", username);
    check_credentials(username, password);
}

fn check_credentials(username: &str, password: &str) {
    println!("Checking credentials for {}", username);
    let hashed = hash_password(password);
    println!("Hashed password: {}", hashed)
}

fn hash_password(password: &str) -> String {
    if password.is_empty() {
        panic!("Password cannot be empty");
    }
    format!("hashed_{}", password)
}
