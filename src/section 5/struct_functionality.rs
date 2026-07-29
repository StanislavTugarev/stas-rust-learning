struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32
}

// Method for struct
// Two requirements for a function to be considered as a method
// - must be inside an implementation block
// - first parameter must be self
impl Car {
    // first form of self a method could take - an immutable reference to self
    fn display_car_info(&self) {
        println!(
            "Owner: {}, Year: {}, Price: {}",
            self.owner, self.year, self.price
        );
    }

    // second form - a mutable reference of self
    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    } 

    // third form - an owned form of self
    fn sell(self) -> Self {
        // refers to the implementing type
        self
    }

    // associated functions
    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }

    // associated function: new (constructor)
    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year: year,
            fuel_level: 0.0,
            price: 0
        }
    }
}

fn main() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        price: 5000
    };
    my_car.display_car_info();

    my_car.refuel(10.5);
    let new_owner = my_car.sell(); // we can't use my_car after this
    let new_car = Car::new(String::from("XYZ"), 2020);
}
