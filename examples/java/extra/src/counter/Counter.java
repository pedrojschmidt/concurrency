package counter;

public class Counter {
    int value = 0;

    void increment() {
        int localCounter = value;
        System.out.println(threadName() + " reads counter as: " + localCounter);

        localCounter = localCounter + 1;

        value = localCounter;
        System.out.println(threadName() + " updated counter to: " + value);
    }

    // It is synchronized, so only one thread can execute this method at a time.
    synchronized void incrementSync() {
        int localCounter = value;
        System.out.println(threadName() + " reads counter as: " + localCounter);

        localCounter = localCounter + 1;

        value = localCounter;
        System.out.println(threadName() + " updated counter to: " + value);
    }

    private String threadName() {
        return Thread.currentThread().getName();
    }
}