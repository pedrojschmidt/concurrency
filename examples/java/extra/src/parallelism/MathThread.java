package parallelism;

import java.util.function.Supplier;

public class MathThread extends Thread {
    private final Supplier<Double> expression;
    private double result;

    public MathThread(Supplier<Double> expression) {
        this.expression = expression;
    }

    public void run() {
        // When running the thread save the result
        result = expression.get();
    }

    public double getValue() {
        return result;
    }
}