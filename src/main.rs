use std::println;

// structs with references needs lifetime to its references
struct ArrayProcesor<'a> {
    data: &'a [i32]
}

// impl needs lifetime annotation if we use generics inside the struct
impl<'a> ArrayProcesor<'a> {
    // lifetime parameter of self assigned to output parameter (3rd rule)
    fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] {
        let previous_data = self.data;
        self.data = new_data;
        previous_data
    }
}

fn main() {
    let mut some_data = ArrayProcesor {data: &[4, 5 ,6]};

    let previous_data = some_data.update_data(&[5, 8, 10]);
    println!("previous data: {:?}", previous_data);
    println!("new data: {:?}", some_data.data);
}