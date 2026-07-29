fn main() {
    let int1 = 5;
    let picked_value;
    // in the inner scope int2 doesn't live long enough
    // {
    let int2 = 10;
    picked_value = picking_int(&int1, &int2);
    // }
    println!("{picked_value}");
}

// fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
fn picking_int(i: &i32, j: &i32) -> &'static i32 {
    // we can't return the reference to x, cause it lives only inside the scope of this function
    let x = 6;
    // static lifetime is equal to the entire duration of the program
    let y: &'static i32 = &6;
    // if rand::random() { i } else { j }
    y
}
