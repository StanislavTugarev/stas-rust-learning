// struct ArrayProcessor<'a> {
//     // lifetime elisions are not defined to structs
//     // as long as it struct
//     data: &'a [i32],
// }

// impl<'a> ArrayProcessor<'a> {
//     // the output doesn't need the lifetime reference due to the 3rd rule
//     fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] {
//         // expanded by compiler to this form
//         // fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &[i32] {
//         let previous_data = self.data;
//         self.data = new_data;
//         previous_data
//     }
// }

// fn main() {
//     let mut some_data = ArrayProcessor { data: &[4, 5, 6] };
//     let previous_data = some_data.update_data(&[5, 6, 7]);
//     println!("{:?}", previous_data);
//     println!("{:?}", some_data.data);
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
