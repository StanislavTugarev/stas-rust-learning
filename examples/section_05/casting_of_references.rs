fn main() {
    // Casting between references
    let x = 5;
    let y = x as f32;

    // Casting immutable reference into mutable reference is not allowed
    // Casting mutable reference into immutable reference is allowed
    let mut data = 42;
    let mutable_ref = &mut data;
    let immutable_ref = mutable_ref as &i32; // a mutable reference can be casted to an immutable reference - reborrowing

    // Assignment of references
    let mut str = String::from("");
    let ref_str_1 = &str;
    let ref_str_2 = ref_str_1; // references are stack allocated, they're copied and not moved
    println!("{ref_str_1}") // mutable references are moved and not copied
}
