use chrono::{Local, Timelike};

fn main() {
    let name = Some("alice");
    let upper = name.map(|s| s.to_uppercase());

    let name: Option<&str> = None;
    let upper = name.map(|s| {
        s.to_uppercase();
        println!("Inside the closure");
    });

    let input = Some(4);
    let result = match input {
        Some(value1) => match add_five_if_even(value1) {
            Some(final_value) => Some(final_value),
            None => None,
        },
        None => None,
    };

    let result = input
        .and_then(|x| add_five_if_even(x))
        .and_then(|x| divide_by_two_if_even(x));

    let primary = Some(1);
    let alternative = Some(10);
    let result = primary.or(alternative);

    let env_value = std::env::var("CONFIG_PATH").ok();
    let file_value = Some("configuration value from a file".to_string());
    let default_value = Some("default configuration".to_string());

    let config = env_value.or(file_value).or(default_value);

    let cached_result = Some(1);
    let resolved_result = cached_result.or_else(|| {
        println!("Computing expensive alternative value...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        Some(42)
    });

    let child_age = Some(10);
    let is_adult = child_age.filter(|&a| a >= 18);

    let user = Some("Alice");
    let name = user.unwrap_or("Guest");

    let greetings_for_user: Option<&str> = None;
    let greetings = greetings_for_user.unwrap_or_else(|| {
        let hour = Local::now().hour();
        if hour < 12 {
            "Good Morning"
        } else if hour < 18 {
            "Good Afternoon"
        } else {
            "Good Evening"
        }
    });
}

fn add_five_if_even(n: u32) -> Option<u32> {
    if n % 2 == 0 { Some(n + 5) } else { None }
}

fn divide_by_two_if_even(n: u32) -> Option<u32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}
