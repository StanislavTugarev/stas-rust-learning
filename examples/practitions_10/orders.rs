trait Order {
    fn label(&self) -> String;
    fn cost(&self) -> f64;
}

struct Product {
    name: String,
    price: f64,
}

struct Service {
    description: String,
    rate: f64,
    hours: u32,
}

impl Order for Product {
    fn label(&self) -> String {
        self.name.clone()
    }

    fn cost(&self) -> f64 {
        self.price
    }
}

impl Order for Service {
    fn label(&self) -> String {
        self.description.clone()
    }

    fn cost(&self) -> f64 {
        self.rate * self.hours as f64
    }
}

fn print_receipt(item: &Box<dyn Order>) {
    println!("The {} costs {}", item.label(), item.cost());
}

// fn expensive_order<T: Order, U: Order>(item_1: &T, item_2: &U) {
//     if item_1.cost() == item_2.cost() {
//         println!("Prices are equal")
//     } else {
//         match item_1.cost() > item_2.cost() {
//             true => print_receipt(item_1),
//             false => print_receipt(item_2),
//         }
//     }
// }

fn main() {
    let product_1 = Product {
        name: "product 1".to_string(),
        price: 4.5,
    };
    let service_1 = Service {
        description: "service_1".to_string(),
        rate: 3.5,
        hours: 3,
    };

    // print_receipt(&product_1);
    // print_receipt(&service_1);

    // expensive_order(&product_1, &service_1);

    let orders: Vec<Box<dyn Order>> = vec![Box::new(product_1), Box::new(service_1)];
    let price = orders.iter().map(|x| x.cost()).sum::<f64>();
    println!("Total price is {price}");

    let filtered_orders = orders
        .into_iter()
        .filter(|x| x.cost() > 10.0)
        .collect::<Vec<Box<dyn Order>>>();

    for order in filtered_orders {
        print_receipt(&order);
    }
}
