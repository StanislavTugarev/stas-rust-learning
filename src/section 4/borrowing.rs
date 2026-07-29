/*
- Borrowing - doesn't take ownership and prevent unnecessary memory usage
- Borrowing rules:
1. At any time, you can have either one mutable reference or any number of immutable references
2. Refernces must always be valid
*/

fn main() {
    let mut vec_1 = vec![4, 5, 6];
    let ref1 = &mut vec_1;
    // let ref2 = &mut vec_1; // cannot borrow `vec_1` as mutable more than once at a time
    // println!("ref1 {:?}, ref2 {:?}", ref1, ref2);

    let mut vec_1 = vec![4, 5, 6];
    let ref1 = &vec_1;
    let ref2 = &vec_1;
    println!("ref1 {:?}, ref2 {:?}", ref1, ref2);

    // let vec_2 = {
    //     let vec_3 = vec![1,2,3];
    //     &vec_3
    // }; // reference to vec_3 doesn't live long enough

    let mut vec_1 = vec![1, 2, 3];
    let ref1 = &vec_1;
    borrows_vec(ref1); // doesn't take ownership, cause we borrow it

    let ref2 = &mut vec_1;
    mutably_borrows_vec(ref2);
    println!("vec 1 is {:?}", vec_1);
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("vec is {:?}", vec);
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(10);
}
