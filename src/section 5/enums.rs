// Enums: multiple variants of a type

// Enums vs Structs:
// Struct fields have types
// Enum variants have no types
// enum WeekDays {
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sunday,
// }
// fn main() {
//     let day = "Saturday";

//     let week_days = vec![
//         "Monday",
//         "Tuesday",
//         "Wednesday",
//         "Thursday",
//         "Friday",
//         "Saturday",
//         "Sunday",
//     ];
//     let day = week_days[6];

//     let day = WeekDays::Sunday;
// }

enum TravelType {
    Car(f32), // we can provide a type but it not necessary
    Train(f32),
    Aeroplane(f32),
}

impl TravelType {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Aeroplane(miles) => miles * 5.0,
        };
        allowance
    }
}

fn main() {
    let participant = TravelType::Car(60.0); // we can store a value if we provide the type in an enum
    TravelType::travel_allowance(&participant);
    println!(
        "Allowance of participant is: {}",
        participant.travel_allowance()
    );
}
