use std::sync::RwLock;

pub struct BankAccountRW {
    balance: RwLock<f64>,
}

impl BankAccountRW {
    pub fn new(initial_balance: f64) -> BankAccountRW {
        BankAccountRW {
            balance: RwLock::new(initial_balance),
        }
    }

    pub fn deposit(&self, amount: f64) {
        if let Ok(mut balance) = self.balance.write() {
            *balance += amount;
            println!("Deposited: {}", amount);
        }
    }

    pub fn withdraw(&self, amount: f64) {
        if let Ok(mut balance) = self.balance.write() {
            if *balance >= amount {
                *balance -= amount;
                println!("Withdrawn: {}", amount);
            } else {
                println!("Insufficient balance for withdrawal");
            }
        }
    }

    pub fn get_balance(&self) -> f64 {
        *self.balance.read().unwrap()
    }
}
