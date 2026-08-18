use std::println;

struct Excerpt<'a> {
    part: &'a str
}

fn main(){
    let a = "123";
    let b = "1234";
    let longest_str = longest(a, b);
    let longest_struct = Excerpt {
        part: longest_str,
    };
    println!("{longest_str}")
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    match a.chars().count() > b.chars().count() {
        true => a,
        false => b
    }
}