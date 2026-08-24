fn main() {
    // stack allocated data
    // lifetime ends in the end of the main funcrion
    let i = 5;
    {
        // lifetime ends in the end of the scope
        let i = 6;
    }
    let j = i;
    println!("i: {i}");

    // heap allocated data
    let str_1 = "abc".to_string();
    let mut str_2 = str_1; // end of the lifetime of str_1
    let mut str_11 = str_2.as_mut_str();
    // we can't use str_1 anymore, the ownership of the value is gone to the str_2

    let i;
    {
        let j = 5;
        i = &j; // we can't use the i outside the scope because the value of doens't live long enough. a dangling reference
    }
    // println!("i: {i}");

    let mut vec_1 = vec![6, 5, 8, 9];
    // non-lexical lifetimes
    // we can have either one mutable reference or multiple immutable
    // lifetime of the reference doesn't tight to scope, we don't use it after creating a mutable reference so it's lifetime ends here
    let ref_1: &_ = &vec_1;
    println!("ref 1: {:?}", ref_1); // lifetime of ref_1 ends here
    let ref_2 = &mut vec_1;
    ref_2.push(3);
    println!("ref 2: {:?}", ref_2);
    let ref_1 = &vec_1;
}
