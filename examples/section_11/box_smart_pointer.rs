#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // we can't use List directly, because compiler need to know the size of value
    Nil,
}


fn main(){
    let x = 0.625; // value stores in stack
    let y = Box::new(x); // value stores in heap
    let z = &x; // point to some memory on stack

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}", list)
}