package bank_account;

import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class BankAccountWithLock {
    private double balance;
    private final Lock lock = new ReentrantLock();

    public BankAccountWithLock(double initialBalance) {
        balance = initialBalance;
    }

    //Method to deposit money with lock
    public void deposit(double amount) {
        lock.lock();
        try {
            if (amount > 0) {
                balance += amount;
                System.out.println("Deposited: " + amount);
            }
        } finally {
            lock.unlock();
        }
    }

    //Method to withdraw money with lock
    public void withdraw(double amount) {
        lock.lock();
        try {
            if (amount > 0 && balance >= amount) {
                balance -= amount;
                System.out.println("Withdrawn: " + amount);
            } else {
                System.out.println("Insufficient balance for withdrawal");
            }
        } finally {
            lock.unlock();
        }
    }

    public double getBalance() {
        lock.lock();
        try {
            return balance;
        } finally {
            lock.unlock();
        }
    }
}
