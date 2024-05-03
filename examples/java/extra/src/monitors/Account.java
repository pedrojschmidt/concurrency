package monitors;

public class Account {
    double balance;
    synchronized public void withdraw(double amount) {
        balance -= amount;
    }

    synchronized public void deposit(double amount) {
        balance += amount;
    }

//  • The synchronize keyword in a method creates a monitor over the current object.
//	• Equivalent to synchronized(this) { }
//	• synchronize in an static method creates a monitor over the class object.
//  • Equivalent to synchronized(X.class)
}
