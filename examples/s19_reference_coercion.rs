// reference coercion
// &mut T -> &T
fn main() {
    let mut s = "Some String".to_string();
    let mut_ref = &mut s;
    let s1 = accepts_string(mut_ref); //reborrow from mut_ref

    // we can't update the mut_ref while s1 is active
    // mut_ref.push('h');

    println!("mut_ref: {}, immut_ref: {}", mut_ref, s1)
}

fn accepts_string(s: &String) -> &String {
    println!("{s}");
    s
}
