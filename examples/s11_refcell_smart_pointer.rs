use std::{cell::RefCell, rc::Rc};

// refcell enforces borrowing rules at runtime instead of compile time
fn main() {
    // let mut x = 50;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;

    // println!("{} {}", x1, x2);

    let a = RefCell::new(10);
    {    
        let b = a.borrow();
        let c = a.borrow();
    }  // scope ends, automatically drops b and c

    // we can drop the value, then code executes
    // drop(b);
    // drop(c);

    // check the borrowing rules during the runtime instead of compile time
    // do not use non-lexical lifetimes
    let d = a.borrow_mut();

    // error in runtime:
    // thread 'main' (326394) panicked at src/main.rs:14:15:
    // RefCell already borrowed
    // println!("{} {}", b, c);

    // a: RefCell { value: <borrowed> }
    drop(d);
    println!("a: {:?}", a);

    let a = RefCell::new(10);
    let mut b = a.borrow_mut(); // using refcell, we can mut immutable variable
    *b = 15;
    drop(b);
    println!("{:?}", a);

    let a = Rc::new(RefCell::new("C++".to_string()));
    let b = Rc::clone(&a);

    *b.borrow_mut() = "Rust".to_string();
    println!("{:?}", a);
} 