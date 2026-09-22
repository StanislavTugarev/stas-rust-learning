use rust_test::Account;
use rust_test::Bank;
fn main() {
    let customer_1 = Account::new(1, String::from("First"), 1000.0);
    let customer_2 = Account::new(2, String::from("Second"), 300.0);
    let mut database = Bank::new();
    database.add_customer(customer_1).add_customer(customer_2);

    database.print_result(1, 2);

    let res = database.trasfer(1, 2, 700.0);
    match res {
        Ok(_) => println!("Successfull transaction"),
        Err(msg) => println!("An error occured: {}", msg),
    }
    database.print_result(1, 2);

    let res = database.trasfer(1, 2, 700.0);
    match res {
        Ok(_) => println!("Successfull transaction"),
        Err(msg) => println!("An error occured: {}", msg),
    }
    database.print_result(1, 2);
}
