enum Shape {
    Circle {radius: f64},
    Rectangle {width: f64, height: f64},
    Triangle {base: f64, height: f64},
}

impl Shape {
    fn area(&self) -> f64 {
        let area: f64 = match self {
            Shape::Circle { radius } => radius * radius * 3.14,
            Shape::Rectangle { width, height }  => width * height,
            Shape::Triangle { base, height } => base * height / 2.0,
        };
        area
    }

    fn try_new_circle(radius: f64) -> Result<Shape, String>{
        match radius {
            ..0.0 => Err("Radius is less than 0".to_string()),
            _=> Ok(Shape::Circle { radius }),
        }
    }
}

fn main() {
    let circle_1 = Shape::try_new_circle(5.0);
    let circle = match circle_1 {
        Err(msg) => { println!("{}", msg); Shape::Circle { radius: 0.0 }},
        Ok(circle) => circle,
    };

    let circle_area = circle.area();
    println!("circle area is {}", circle_area);

    let figures: Vec<Shape> = vec![Shape::Circle { radius: 2.0 }, Shape::Rectangle {width: 3.0, height: 4.0}, Shape::Triangle {base: 2.0, height: 3.0}];
    let areas: f64 = figures.iter().map(|x| x.area()).sum();
    println!("sum of areas: {}", areas)
}