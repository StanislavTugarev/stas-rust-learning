/* Ownership Basics
1. Each value has a variable that's it "owner"
2. A value can have only one owner at a time
3. If the owner goes out of scope, the value is cleaned up
*/

fn main() {
    let s1 = String::from("worlds");
    // let s2 = s1; // s2 takes s1 params and value and now it's owner, s1 is not available after this
    let s2 = s1.clone(); // clone the s1 params and clone the heap value

    {
        let s3 = s1;
        // we can't access this value out of the scope
    }

    println!("s1 is {s2}");

    let x = 15;
    let y = x;
    println!("x is {x}"); // the primitive types value saves in stack, not a heap, so we don't give the ownership x to y

    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1); //passing a variable to a function has the same effect as assigning it to another variable

    let vec_2 = gives_ownership();
    println!("vec 2 is {:?}", vec_2);

    let vec_3 = takes_and_gives_ownership(vec_2); // we can't access vec 2 after this, cause we give its ownership to a function, however we recieve a vec 3

    // primitive types
    let x = 10;
    stack_function(x);
    println!("x in main is {x}");
}

fn takes_ownership(vec: Vec<i32>) {
    println!("vec is {:?}", vec);
    // value is cleaned up after this scope
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn stack_function(mut var: i32) {
    var = 56;
    println!("function x is {var}");
}
