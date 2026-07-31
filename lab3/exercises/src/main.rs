mod traits;
mod custom_error;
mod stretch_goal3;

#[derive(Debug, Clone)]
pub struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Associated function (constructor)
    pub fn new(owner: &str, initial_balance: f64) -> Self {
        BankAccount {
            owner: owner.to_string(),
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
    }

    // TODO 1: Implement `withdraw(&mut self, amount: f64) -> Result<(), String>`
    //         Return Err("Insufficient funds".into()) if amount > balance
    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            Err("Insufficient funds".into())
        } else {
            self.balance -= amount;
            println!("Withdrew ${:.2}. New balance: ${:.2}", amount, self.balance);
            Ok(())
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

pub fn run() {
    let mut acc = BankAccount::new("Alice", 1000.0);
    acc.deposit(500.0);

    match acc.withdraw(200.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(msg) => println!("Error: {}", msg),
    }

    println!("Final balance: ${:.2}", acc.balance());
}

fn main() {
    run();
    
    //------------------------------
    
    println!("running exercise B--------");
    traits::run();

    //-------------------------------
   
    println!("running exercise C------------");
    custom_error::run();

    //----------------------------------
    println!("Running the stretch goal-------------");
    stretch_goal3::run();

}
