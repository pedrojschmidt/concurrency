package semaphores;

import java.util.concurrent.Semaphore;

import static java.lang.Thread.sleep;

public class Elevator { // Counting semaphore
    Semaphore semaphore;

    public Elevator(int capacity) {
        semaphore = new Semaphore(capacity);
    }

    public void enter(String name) throws InterruptedException {
        semaphore.acquire();
        System.out.println(name + " entered the elevator");
        sleep(1_000); // Simulate elevator ride
        System.out.println(name + " exited the elevator");
        semaphore.release();
    }
}
