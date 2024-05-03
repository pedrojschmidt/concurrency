package monitors;

public class AccountCondVar { // We want to wait that there is enough balance
    double balance;
    synchronized public void withdraw(double amount) throws InterruptedException {
        if (amount <= 0) return;

        while (balance < amount) {
            wait(); // Wait for enough balance
        }
        balance -= amount;
    }

    synchronized public void deposit(double amount) {
        if (amount > 0) {
            balance += amount;
            notifyAll(); // Notify that some money have been deposited
        }
    }
}
