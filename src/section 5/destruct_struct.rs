struct Point {
    x: i32,
    y: i32,
}

fn print_coord(Point { x, .. }: Point) {
    // Value: p or Point {x: 5, y: 7}
    // Pattern: Point {x, y}

    // we have a direct access to x
    println!("x: {x}");
}
fn main() {
    let p = Point { x: 0, y: 7 };

    // destruct struct parameters
    match p {
        Point { x: 0, y } => println!("On the y-axis at {y}"), // we can't use x because x only uses in pattern matching andis not binded to some value
        Point { x, y: 0 } => println!("On the x-axis at {x}"),
        Point { x, y } => println!("at point ({x} {y})"),
    }
    // first arm
    // value: p or Point {x: 0, y: 7}
    // pattern: Point {x: 0, y}

    let x = 5;
    if let x = 5 {};
    // value: 5
    // pattern: x

    let p = Point { x: 5, y: 7 };
    print_coord(p);
}
