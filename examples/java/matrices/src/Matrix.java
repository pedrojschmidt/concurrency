public class Matrix {
    private final double[][] values;

    public Matrix(double[][] values) {
        this.values = values;
    }

    public double sum() {
        var result = 0.0;
        for (double[] value : values) {
            result += addRow(value);
        }
        return result;
    }

    private double addRow(double[] value) {
        var result = 0.0;
        for (double v : value) {
            result += v;
        }
        return result;
    }


    public static void main(String[] args) {
        Matrix m = create(10, 10_000);

        System.out.println("Sum = " + m.sum());
    }

    private static Matrix create(int rows, int cols) {
        double[][] result = new double[rows][];
        for (int i = 0; i < rows; i++) {
            var row = new double[cols];
            for (int j = 0; j < cols; j++) {
                row[j] = 1.0;
            }
            result[i] = row;
        }
        return new Matrix(result);
    }
}