package bank_account;

public class BankAccountSync {
    private double balance;
    private final Object lock = new Object();

    public BankAccountSync(double initialBalance) {
        balance = initialBalance;
    }

    // Synchronized method to deposit money
    public void deposit(double amount) {
        synchronized (lock) {
            if (amount > 0) {
                balance += amount;
                System.out.println("Deposited: " + amount);
            }
        }
    }

    // Synchronized method to withdraw money
    public void withdraw(double amount) {
        synchronized (lock) {
            if (amount > 0 && balance >= amount) {
                balance -= amount;
                System.out.println("Withdrawn: " + amount);
            } else {
                System.out.println("Insufficient balance for withdrawal");
            }
        }
    }

    public double getBalance() {
        synchronized (lock) {
            return balance;
        }
    }
}
