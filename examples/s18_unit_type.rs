fn main() {
    let x = ();
    let y = f1();

    let z = println!("Hello");

    let mut vec: Vec<()> = Vec::with_capacity(0);
    vec.push(());
    vec.push(());
    vec.push(());
    assert_eq!(3, vec.len());
    println!("{}", vec.capacity());
}

// fn f1() {}
fn f1() -> () {}

fn division_status(divident: f64, divisor: f64) -> Result<(), String> {
    let answer = match divisor {
        0.0 => Err("Error: Division by zero".to_string()),
        _ => {
            println!("The division is invalid");
            Ok(())
        }
    };
    answer
}
