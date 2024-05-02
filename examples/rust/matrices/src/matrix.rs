use std::thread;

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
       thread::scope(|s| {
            let mut threads = Vec::new();
            for row in &self.0 {
                threads.push(s.spawn(move || self.add_row(&row)));
            }

            let mut result = 0.0;
            for thread in threads {
                result += thread.join().unwrap();
            }
            result
        })
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
        let mut result = Vec::new();

        for i in 0..rows {
            let mut row = Vec::new();
            for j in 0..cols {
                row.push(self.0[i][j] + other.0[i][j]);
            }
            result.push(row);
        }
        Matrix(result)
    }

    pub fn add_parallel(&self, other: &Matrix) -> Matrix {
        let rows = self.rows();
        let cols = self.columns();

        thread::scope(|s| {
            let threads: Vec<_> = (0..rows)
                .map(|i| {
                    s.spawn(move || {
                        (0..cols).map(|j| self.0[i][j] + other.0[i][j]).collect()
                    })
                })
                .collect();

            Matrix(threads.into_iter()
                .map(|t| t.join().unwrap())
                .collect()
            )
        })
    }


}