use std::sync::Mutex;

pub struct BankAccount {
    balance: Mutex<f64>,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: Mutex::new(initial_balance),
        }
    }

    pub fn deposit(&self, amount: f64) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
        println!("Deposited: {}", amount);
    }

    pub fn withdraw(&self, amount: f64) {
        if let Ok(mut balance) = self.balance.lock() {
            if *balance >= amount {
                *balance -= amount;
                println!("Withdrawn: {}", amount);
            } else {
                println!("Insufficient balance for withdrawal");
            }
        }
    }

    pub fn get_balance(&self) -> f64 {
        *self.balance.lock().unwrap()
    }
}
