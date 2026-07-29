struct BankAccount {
    balance: i32,
    owner: String,
}

impl BankAccount {
    fn new(owner: String, initial_balance: i32) -> Self {
        println!("Account opened successfully");
        Self {
            balance: initial_balance,
            owner,
        }
    }

    fn change_owner(mut self, new_owner: String) -> Self {
        self.owner = new_owner;
        self
    }

    fn check_balance(&self) {
        println!("{}'s balance: ${}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: i32) -> &mut Self {
        self.balance += amount;
        println!("Deposited ${} to {}'s account", amount, self.owner);
        self
    }

    fn withdraw(&mut self, amount: i32) -> &mut Self {
        if self.balance >= amount {
            self.balance -= amount;
            println!("withdrew ${} from {}'s account", amount, self.owner);
        } else {
            println!(
                "Insufficient funds for withdrawl in {}'s account",
                self.owner
            )
        };
        self
    }

    fn view_owner(&self) -> &Self {
        println!("Account owner: {}", self.owner);
        self
    }
}

/*
Self type       Method & Notes

self            change_owner(mut self) -> Self

&self           check_balance(&self)
                view_owner(&self) -> &Self

&mut self       deposit(&mut self) -> &mut Self
                withdraw(&mut self) -> &mut Self

No self         new() -> Self
 */

// Method chaining depends on how each method recieves and return back self
fn main() {
    let mut account = BankAccount::new(String::from("Michael"), 4000);

    // 1. Methods that doeens't return anything
    account.check_balance(); // we can't grow the chain because check_balance returns nothing

    // 2. Methods that return a &mut Self
    // &mut Self -> chained with methods requiring &mut self or &self
    account.deposit(100).withdraw(50).view_owner(); //return of deposit satisfies withdraw input. we can add view_owner because &mut can cast to immutable reference

    // 3. Methods that return &Self
    // &Self -> chained with methods requiring &self
    account.view_owner().check_balance();

    // 4. Methods that return an owned form of Self
    // Self -> chained with methods accepting any of the three forms of self
    account
        .change_owner(String::from("new owner"))
        .change_owner(String::from("another owner"))
        .deposit(100)
        .view_owner();
}
