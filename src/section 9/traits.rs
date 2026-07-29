struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

struct Circle {
    radius: f32,
}

// impl Square {
//     fn calculate_area(&self) {
//         println!("The area is: {}", self.side * self.side)
//     }
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.length * self.width
//     }
// }

// supertrait
trait Draw {
    fn draw_object(&self);
}

// marker trait - trait without methods
trait OtherTrait {}

trait SomeOtherTrait {}

trait Shape: Draw + OtherTrait + SomeOtherTrait {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimiter not implemented");
        0.0
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("Reactangle area: {}", area_of_rect);
        area_of_rect
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("Rectangle perimeter: {}", perimeter_of_rect);
        perimeter_of_rect
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Drawing Rectangle");
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("Drawing Square");
    }
}

impl OtherTrait for Rectangle {}

impl OtherTrait for Square {}

impl SomeOtherTrait for Rectangle {}

impl SomeOtherTrait for Square {}

// returning type determins in execution time, not in compile time
fn returns_shape(dimension: Vec<f32>) -> Box<dyn Shape> {
    if dimension.len() == 1 {
        let sq = Square {
            side: dimension[0],
            line_width: 5,
            color: String::from("Red"),
        };
        Box::new(sq)
    } else {
        let rect = Rectangle {
            length: dimension[0],
            width: dimension[1],
            line_width: 5,
            color: String::from("Red"),
        };
        Box::new(rect)
    }
}

// a trait bound
// supertraits included in Shape
// static dispatch - resolves at compile time, rust generates a specific copy of the function for every concrete type we use
fn shape_properties_static<T: Shape>(object: T) {
    object.area();
    object.perimeter();
}

fn shape_properties_rect(object: Rectangle) {
    object.area();
    object.perimeter();
}

fn shape_properties_sq(object: Square) {
    object.area();
    object.perimeter();
}

// dyn - dynamic dispatch
// resolves at execution time
fn shape_properties_dynamic(object: Box<dyn Shape>) {
    object.area();
    object.perimeter();
}

fn main() {
    let r1 = Rectangle {
        width: 5.0,
        length: 4.0,
        line_width: 1,
        color: String::from("Red"),
    };

    let s1 = Square {
        side: 3.2,
        line_width: 1,
        color: String::from("Red"),
    };

    let c1 = Circle { radius: 5.0 };

    r1.area();
    s1.area();

    r1.perimeter();
    s1.perimeter();

    shape_properties_dynamic(Box::new(r1));
    shape_properties_static(s1);
}
