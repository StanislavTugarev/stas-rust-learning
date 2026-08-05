pub use self::vault::Account;
pub use self::vault::Bank;

mod vault {
    use std::collections::HashMap;

    pub struct Account {
        id: u32,
        owner: String,
        pub balance: f64,
    }

    pub struct Bank {
        pub db: HashMap<u32, Account>,
    }

    impl Account {
        pub fn new(id: u32, owner: String, balance: f64) -> Self {
            Account { id, owner, balance }
        }
    }

    impl Bank {
        pub fn new() -> Self {
            Bank { db: HashMap::new() }
        }

        pub fn add_customer(&mut self, customer: Account) -> &mut Self {
            self.db.entry(customer.id).or_insert(customer);
            self
        }

        pub fn trasfer(&mut self, from_id: u32, to_id: u32, amount: f64) -> Result<(), String> {
            if self.db.contains_key(&from_id) && self.db.contains_key(&to_id) {
                if self.db.get(&from_id).unwrap().balance >= amount {
                    self.db.get_mut(&from_id).unwrap().balance -= amount;
                    self.db.get_mut(&to_id).unwrap().balance += amount;
                    Ok(())
                } else {
                    Err(String::from("Not enough money for transfer"))
                }
            } else {
                Err(String::from("Account doesn't exist"))
            }
        }

        pub fn print_result(&self, id_1: u32, id_2: u32) {
            println!(
                "customer {} has ${}, customer {} has ${}",
                self.db.get(&id_1).unwrap().owner,
                self.db.get(&id_1).unwrap().balance,
                self.db.get(&id_2).unwrap().owner,
                self.db.get(&id_2).unwrap().balance,
            );
        }
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    #[test]
    fn successfull_transaction() {
        let customer_1 = Account::new(1, String::from("First"), 1000.0);
        let customer_2 = Account::new(2, String::from("Second"), 300.0);
        let mut database = Bank::new();
        database.add_customer(customer_1).add_customer(customer_2);
        let res = database.trasfer(1, 2, 700.0);
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn error_not_enough_money() {
        let customer_1 = Account::new(1, String::from("First"), 1000.0);
        let customer_2 = Account::new(2, String::from("Second"), 300.0);
        let mut database = Bank::new();
        database.add_customer(customer_1).add_customer(customer_2);
        let res = database.trasfer(1, 2, 1700.0);
        assert_eq!(res, Err(String::from("Not enough money for transfer")))
    }

    #[test]
    fn error_account_does_not_exist() {
        let customer_1 = Account::new(1, String::from("First"), 1000.0);
        let customer_2 = Account::new(2, String::from("Second"), 300.0);
        let mut database = Bank::new();
        database.add_customer(customer_1).add_customer(customer_2);
        let res = database.trasfer(3, 2, 1700.0);
        assert_eq!(res, Err(String::from("Account doesn't exist")))
    }
}
