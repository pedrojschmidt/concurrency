use std::sync::Mutex;
use std::thread;
use bank_account::BankAccount;
use crate::bank_account_rw::BankAccountRW;

mod bank_account;
mod bank_account_rw;

fn main() {

    let  counter = Mutex::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            *counter.lock().unwrap() += 1;
        });

        s.spawn(|| {
            *counter.lock().unwrap() += 1;
        });
    });



    let balance = try_mutex_account();
    println!("{balance}");
    if balance != 50.0 {
        panic!("Incorrect balance");
    }
    let balance = try_rw_accout();
    println!("{balance}");
    if balance != 50.0 {
        panic!("Incorrect balance");
    }
}

fn try_mutex_account() -> f64 {
    let account = BankAccount::new(150.0);

    thread::scope(|s| {
        s.spawn(|| {
            account.withdraw(100.0);
            account.deposit(50.0);
        });

        s.spawn(|| {
            account.withdraw(100.0);
            account.deposit(50.0);
        });
    });
    account.get_balance()
}
fn try_rw_accout() -> f64 {
    let account = BankAccountRW::new(150.0);

    thread::scope(|s| {
        s.spawn(|| {
            account.withdraw(100.0);
            account.deposit(50.0);
        });

        s.spawn(|| {
            account.withdraw(100.0);
            account.deposit(50.0);
        });
    });
    account.get_balance()
}
