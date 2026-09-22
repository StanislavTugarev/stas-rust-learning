use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::atomic::{AtomicUsize, Ordering},
};

static FMT_CALLS: AtomicUsize = AtomicUsize::new(0);

struct Node {
    // next: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

struct Show<'a, T>(&'a T);

impl<'a, T: std::fmt::Debug> std::fmt::Display for Show<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.0, f)
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let call_no = FMT_CALLS.fetch_add(1, Ordering::Relaxed) + 1;
        eprintln!("fmt call #{call_no}");
        write!(f, "Node {{ next: ")?;
        self.next.fmt(f)?;
        write!(f, " }}")
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
        // next: Some(Rc::clone(&a)),
        next: Some(Rc::clone(&a)), // next: Some(Rc::clone(&a)),
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
        next: Some(Rc::clone(&b)),
        // next: Some(Rc::clone(&b)),
    }));

    (*a).borrow_mut().next = Some(Rc::clone(&c));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        println!("After crating cycle \n a count: {:?}", a);
    }));
    if let Err(err) = result {
        println!("caught panic: {:?}", err);
    }
    println!("Node::fmt calls: {}", FMT_CALLS.load(Ordering::Relaxed));
    // println!("b count: {:?}", Rc::strong_count(&b));
    // println!("c count: {:?}", Rc::strong_count(&c));

    // println!("a: {}", Show(&a));
}
