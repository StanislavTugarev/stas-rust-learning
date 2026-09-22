use std::{fmt::Debug, fmt::Display};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "Color {{hex: \"#{:X}{:X}{:X}\", 
            red: {},
            blue: {},
            green: {}",
                self.red, self.green, self.blue, self.red, self.green, self.blue
            )
        } else {
            write!(f, "Color (#{:X}{:X}{:X})", self.red, self.green, self.blue)
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB ({} {} {}) - hex: #{:x}{:x}{:x}",
            self.red, self.green, self.blue, self.red, self.green, self.blue
        )
    }
}

fn main() {
    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: -3, y: 4 },
        Point { x: 5, y: 6 },
    ];

    for point in &points {
        println!("{:?}", point)
        // println!("Point with coordinates ({} {})", point.x, point.y)
    }

    let p = Point { x: 3, y: 4 };
    let q = Point { x: 5, y: 7 };

    let l = Line { start: p, end: q };
    println!("{:?}", l);
    println!("{:#?}", l);

    let magenta = Color {
        red: 255,
        green: 0,
        blue: 255,
    };

    println!("{:?}", magenta);
    println!("{:#?}", magenta);
    println!("{}", magenta)
}
