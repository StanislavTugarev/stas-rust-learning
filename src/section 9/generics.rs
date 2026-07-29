struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

impl Point<i32, i32> {
    fn printing(&self) {
        println!("The values of the coordinates are {}, {}", self.x, self.y);
    }
}

impl Point<f64, f64> {
    fn printing(&self) {
        println!("The values of the coordinates are {}, {}", self.x, self.y)
    }
}

// a free function
fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
    unimplemented!();
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    let p1 = Point { x: 1.0, y: 4.0 };

    let p2 = Point { x: 5, y: 5.0 };

    let origin = Point::new(0, 0);
    let p1 = Point::new(1.0, 4.0);
    let p2 = Point::new(5, 5.0);

    origin.printing();
    p1.printing();

    add_points(&origin, &origin);
    add_points(&p1, &p1);
}
