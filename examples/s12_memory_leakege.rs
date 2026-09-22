use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    // next: Option<Rc<RefCell<Node>>>,
    next: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    println!(
        "a count: {:?}, a weak count: {:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );

    let b = Rc::new(RefCell::new(Node {
        next: Some(Rc::downgrade(&a)),
        // next: Some(Rc::clone(&a)),
    }));
    println!(
        "B is created: \n a count: {:?}, a weak count: {:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b count: {:?}, b weak count: {:?}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );

    let c = Rc::new(RefCell::new(Node {
        next: Some(Rc::downgrade(&b)),
        // next: Some(Rc::clone(&b)),
    }));

    (*a).borrow_mut().next = Some(Rc::downgrade(&c));

    println!("After crating cycle \n a count: {:?}", Rc::strong_count(&a));
    println!("b count: {:?}", Rc::strong_count(&b));
    println!("c count: {:?}", Rc::strong_count(&c));

    // println!("a: {:?}", a);
}
