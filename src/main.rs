use std::format;

trait Describe {
    fn name(&self) -> String;
    fn describe(&self) -> String {
        format!("This is {}", self.name())
    }
}

struct Dog {
    name: String,
    age: u32,
    color: String
}

struct Car {
    mark: String,
    model: String,
    manufacture_date: u32
}

impl Describe for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Describe for Car {
    fn name(&self) -> String {
        format!("{} {}", self.mark, self.model)
    }
}

fn announce<T: Describe>(item: T) {
    item.describe();
}

fn main() {
    let dog = Dog {
        name: "barker".to_string(),
        age: 5,
        color: "brown".to_string(),
    };  
    let car = Car {
        mark: "Ford". to_string(),
        model: "Focus".to_string(),
        manufacture_date: 2002
    };

    let list: Vec<Box<dyn Describe>> = vec![Box::new(dog), Box::new(car)];
    for item in list {
        println!("{}", item.describe());
    }
}