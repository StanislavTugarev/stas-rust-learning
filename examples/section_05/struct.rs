// name-field struct
struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

// tuple struct
struct Point2D(i32, i32);
struct Point3D(i32, i32, i32);

fn main() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        price: 5000,
    };
    let car_year = my_car.year;
    my_car.fuel_level = 30.0;
    // give ownership to the variable
    // Partial move: some portion of data has moved out of struct instance
    // let extracted_owner = my_car.owner;
    let extracted_owner = my_car.owner.clone();

    let another_car = Car {
        owner: String::from("new_name"),
        ..my_car // copy data from other struct. Only for stack allocated
    };

    // Tuple structs
    let point_2d: (i32, i32) = (1, 3);
    let point_3d: (i32, i32, i32) = (4, 10, 13);

    let point1 = Point2D(1, 3);
    let point2 = Point3D(4, 10, 13);

    // Unit-like structs
    struct ABC;
}

fn print_tuple_coordinates(coords: Point2D) {
    println!("x is {}, y is {}", coords.0, coords.1);
}
