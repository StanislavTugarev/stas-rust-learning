use std::rc::Rc;

enum List {
    Cons(i32, Option<Rc<List>>)
}

// Rc enables multiple ownership 
fn main() {
    let d = Rc::new(5);
    let cc = d.clone();
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None))))); // creates a new reference-counted value on the heap
    println!("Reference count after a: {}", Rc::strong_count(&a));
    {
        let b = List::Cons(3, Some(Rc::clone(&a))); // create a new owner and it doesn't copy the inner data
        println!("Reference count after b: {}", Rc::strong_count(&a)); 

        let c = List::Cons(4, Some(Rc::clone(&a)));
        println!("Reference count after c: {}", Rc::strong_count(&a));
    }
    println!("Reference count after scope: {}", Rc::strong_count(&a));
} 