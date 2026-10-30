struct ContainsStr<'a> {
    s: &'a str,
}

fn main() {
    // let with clear type annotation
    let s: &str;
    s = &"Some String".to_string();

    // function argument
    let s = "Some String".to_string();
    accepts_str(&s);

    // initializing of struct field
    let s = ContainsStr {
        s: &"Some String".to_string(),
    };
}

fn accepts_str(s: &str) {
    println!("Hi, {s}!");
}

// return of the function
fn returns_str(s: &String) -> &str {
    println!("Returning, {s}!");
    s
}
