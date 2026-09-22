use std::println;

struct Order {
    name: String, 
    price: f64, 
    paid: bool
}

fn main() {
    let order_1 = Order {
        name: "PC".to_string(),
        price: 1999.99,
        paid: true
    };
    let order_2 = Order {
        name: "Phone".to_string(),
        price: 999.99,
        paid: true
    };
    let order_3 = Order {
        name: "Headphones".to_string(),
        price: 99.00,
        paid: false
    };

    let mut orders: Vec<Order> = vec![order_1, order_2, order_3];
    let sum_of_all: f64 = orders.iter().map(|x| x.price).sum();
    let expensive_order: Vec<String> = orders.iter().filter(|x| x.price > 100.00).map(|x| x.name.clone()).collect();
    let the_most_expensive = orders.iter().max_by(|x, y| x.price.total_cmp(&y.price));
    let the_most_expensive_name = match the_most_expensive {
        Some(item) => item.name.clone(),
        None => "nothing".to_string()
    };
    println!("sum of all orders: {sum_of_all}, orders > 100 : {:?}, the most expensive is {}", expensive_order, the_most_expensive_name);

    let discount = |price: f64| -> f64{
        price /100.00 * 85.00
    };

    apply_discount(&mut orders, discount);

    let sum_of_all: f64 = orders.iter().map(|x| x.price).sum();
    let expensive_order: Vec<String> = orders.iter().filter(|x| x.price > 100.00).map(|x| x.name.clone()).collect();
    let the_most_expensive = orders.iter().max_by(|x, y| x.price.total_cmp(&y.price));
    let the_most_expensive_name = match the_most_expensive {
        Some(item) => item.name.clone(),
        None => "nothing".to_string()
    };
    println!("sum of all orders: {sum_of_all}, orders > 100 : {:?}, the most expensive is {}", expensive_order, the_most_expensive_name);
}

fn apply_discount<F: Fn(f64) -> f64> (orders: &mut Vec<Order>, f: F) {
    orders.iter_mut().filter(|x| !x.paid).for_each(|x| x.price = f(x.price))
}