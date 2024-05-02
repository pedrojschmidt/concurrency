package parallelism;

public class Main {

    public static void main(String[] args) throws InterruptedException {
        double k = 1.0;
        double a = 2.0;
        double t = 3.0;

        double result = evaluateParallel(k, a, t);
        System.out.println("Result: " + result);
    }

    // 2 * k * a * t * exp(-a * t * t)
    static double evaluateParallel(double k, double a, double t) throws InterruptedException {

        var thread1 = new MathThread(() -> 2 * k * a * t);
        var thread2 = new MathThread(() -> Math.exp(-a * t * t));

        thread1.start();
        thread2.start();

        // Wait for both threads to complete
        thread1.join();
        thread2.join();

        // Now get the values and multiply them
        return thread1.getValue() * thread2.getValue();
    }
}
