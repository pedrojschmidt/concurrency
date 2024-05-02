package semaphores;

import java.util.concurrent.Semaphore;

public class Counter { // Binary semaphore
    int value = 0;
    Semaphore semaphore = new Semaphore(1, true);

    void increment() throws InterruptedException {
        semaphore.acquire(); // wait or down or P

        int localCounter = value;
        localCounter = localCounter + 1;

        value = localCounter;
        semaphore.release(); // signal or up or V
    }
}
