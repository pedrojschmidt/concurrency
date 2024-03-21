package org.example;

import java.util.ArrayList;

public class HelloThreads {
    private static void hello(int numberOfThreads) throws InterruptedException {
        var threads = new ArrayList<Thread>();
        for (int i = 0; i< numberOfThreads; i++) {
            threads.add(new Thread(() -> {
                System.out.println("Hello from thread: " + Thread.currentThread().getId());
                System.out.println("Good bye from thread: " + Thread.currentThread().getId());
            }));
        }
        for (Thread thread1 : threads) {
            thread1.start();
        }
        for (Thread thread : threads) {
            thread.join();
        }
    }


    public static void main(String[] args) throws InterruptedException {
        System.out.println("====== Hello Threads    =======");
        hello(10);
        System.out.println("====== Good Bye Threads =======");
    }
}