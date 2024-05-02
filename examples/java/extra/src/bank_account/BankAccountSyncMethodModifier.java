package bank_account;

public class BankAccountSyncMethodModifier {
    private double balance;

    public BankAccountSyncMethodModifier(double initialBalance) {
        balance = initialBalance;
    }

    // Synchronized method to deposit money
    public synchronized void deposit(double amount) {
        if (amount > 0) {
            balance += amount;
            System.out.println("Deposited: " + amount);
        }
    }

    // Synchronized method to withdraw money
    public synchronized void withdraw(double amount) {
        if (amount > 0 && balance >= amount) {
            balance -= amount;
            System.out.println("Withdrawn: " + amount);
        } else {
            System.out.println("Insufficient balance for withdrawal");
        }
    }

    public double getBalance() {
        synchronized (this) {
            return balance;
        }
    }
}
