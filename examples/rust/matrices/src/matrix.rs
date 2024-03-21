
#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl Matrix {
    pub fn rows(&self) -> usize { self.0.len() }
    pub fn columns(&self) -> usize { self.0[0].len() }

    pub fn sum_serial(&self) -> f64 {
        let mut result = 0.0;
        for row in &self.0 {
            result += self.add_row(row);
        }
        return result;
    }

    pub fn sum_parallel(&self) -> f64 {
        todo!("Implement me!")
    }

    fn add_row(&self, row: &Vec<f64>) -> f64 {
        let mut result = 0.0;
        for v in row {
            result += v;
        }
        return result
    }

    pub fn add_serial(&self, other: &Matrix) -> Matrix {
        let rows = self.rows();
        let cols = self.columns();
        let result = (0..rows)
            .map(|i|
                (0..cols)
                    .map(|j| self.0[i][j] + other.0[i][j])
                    .collect()
            )
            .collect();
        Matrix(result)
    }

    pub fn add_parallel(&self, _: &Matrix) -> Matrix {
        todo!("Implement me!")
    }


}