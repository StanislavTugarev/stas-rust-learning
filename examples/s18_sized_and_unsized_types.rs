trait Shape {
    fn print(&self);
}

#[derive(Debug)]
struct Circle;

#[derive(Debug)]
struct Rectangle;

impl Shape for Circle {
    fn print(&self) {
        println!("{:?}", self)
    }
}

impl Shape for Rectangle {
    fn print(&self) {
        println!("{:?}", self)
    }
}

fn main() {
    println!("i32 size is {}", size_of::<i32>());
    println!("(i32, i32) size is: {}", size_of::<(i32, i32)>());
    println!("[i32: 3] size is: {}", size_of::<[i32; 3]>());
    println!("Vec<i32> size is: {}", size_of::<Vec<i32>>());

    struct Point {
        x: bool,
        y: i64,
    }

    println!("Struct size is: {}", size_of::<Point>());
    println!("i32 reference is: {}", size_of::<&i32>());
    println!("i32 mutable reference is: {}", size_of::<&mut i32>());
    println!("Machine word size is: {}", size_of::<&()>());
    println!("Box<i32> is: {}", size_of::<Box<i32>>());
    println!("fn(i32) -> i32 is: {}", size_of::<fn(i32) -> i32>());

    let link = &();

    println!(
        "Size of a reference to sized type: {}",
        size_of::<&[i32; 3]>()
    );
    println!(
        "Size of a reference to unsized type: {}",
        size_of::<&[i32]>()
    );

    let num_1 = &[10, 12, 13];
    let num_2: &[i32] = &[10, 12, 13];

    let mut sum = 0;

    for num in num_1 {
        sum += num
    }

    for num in num_2 {
        sum += num
    }

    println!("Size of &Circle is: {}", size_of::<&Circle>());
    println!("Size of &Rectangle is: {}", size_of::<&Rectangle>());
    println!("Size of &dyn Shape: {}", size_of::<&dyn Shape>());
}

fn my_fn(my_closure: fn(i32) -> i32) {
    my_closure(5);
}
