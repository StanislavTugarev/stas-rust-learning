extern crate rand;
use rand::random_range;
use rust_test::Account;
use rust_test::Bank;

#[test]
fn stress_test() {
    let mut database = Bank::new();
    let mut total_balance: f64 = 0.0;
    for i in 0..100 {
        database.add_customer(Account::new(i, String::from("account #{i}"), 1000.0));
        total_balance += database.db.get(&i).unwrap().balance;
    }
    println!("{}", total_balance);

    for i in 0..50 {
        database.trasfer(random_range(0..=100), random_range(0..=100), 700.0);
    }

    let mut new_balance = 0.0;
    for i in 0..100 {
        new_balance += database.db.get(&i).unwrap().balance;
    }
    println!("{}", new_balance);

    assert_eq!(total_balance, new_balance);
}
