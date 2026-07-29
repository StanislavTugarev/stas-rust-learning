#[derive(Debug)]
struct Km {
    value: u32,
}

#[derive(Debug)]
struct Kmh {
    value: u32,
}

#[derive(Debug)]
struct Miles {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}

// impl Kmh {
//     fn distance_in_three_hours(&self) -> Km {
//         Km {
//             value: self.value * 3,
//         }
//     }
// }

// impl Mph {
//     fn distance_in_three_hours(&self) -> Miles {
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }

// Assosiated type
trait DistanceInThreeHours {
    type Distance;
    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceInThreeHours for Kmh {
    type Distance = Km;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

impl DistanceInThreeHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

fn main() {
    let speed_kmh = Kmh { value: 90 };
    let distance_km = speed_kmh.distance_in_three_hours();

    println!(
        "At {:?}, you will travel {:?} in 3 hours",
        speed_kmh, distance_km
    );

    let speed_mph = Mph { value: 90 };
    let distance_miles = speed_mph.distance_in_three_hours();

    println!(
        "At {:?}, you will travel {:?} in 3 hours",
        speed_mph, distance_miles
    )
}
