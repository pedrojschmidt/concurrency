package counter;

public class Main {
    public static void main(String[] args) {
        var counter = new Counter();

        Thread t1 = new Thread(counter::increment, "Thread 1");
        Thread t2 = new Thread(counter::increment, "Thread 2");

        startAll(t1, t2);
        joinAll(t1, t2);

        System.out.println("Expected value: 2, actual value: " + counter.value);

        // Output:
        // Thread 1 reads counter as: 0
        // Thread 2 reads counter as: 0
        // Thread 1 updated counter to: 1
        // Thread 2 updated counter to: 1
        // Expected value: 2, actual value: 1
    }

    private static void startAll(Thread t1, Thread t2) {
        t1.start();
        t2.start();
    }

    private static void joinAll(Thread t1, Thread t2) {
        try {
            t1.join();
            t2.join();
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
    }
}
