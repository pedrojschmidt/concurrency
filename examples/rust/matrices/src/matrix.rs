use std::thread;

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl Matrix {
    pub fn rows(&self) -> usize { self.0.len() }
    pub fn columns(&self) -> usize { self.0[0].len() }

    pub fn sum_serial(&self) -> f64 {
        let mut result = 0.0;
        for i in 0..self.rows() {
            result += self.add_row(i);
        }
        return result;
    }

    pub fn sum_parallel(&self) -> f64 {
        return thread::scope(|s| {
            let mut threads = Vec::new();
            for i in 0..self.rows() {
                let thread = s.spawn(move || self.add_row(i));
                threads.push(thread);
            };

            let mut result = 0.0;
            for thread in threads {
                result += thread.join().unwrap();
            }
            result
        });
    }

    fn add_row(&self, i: usize) -> f64 {
        let mut row = 0.0;
        for j in 0..self.columns() {
            row += self.0[i][j];
        }
        row
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

    pub fn add_parallel(&self, other: &Matrix) -> Matrix {
        thread::scope(|s| {
            let rows = self.rows();
            let cols = self.columns();

            let threads: Vec<_> = (0..rows)
                .map(|i| {
                    s.spawn(move || {
                        (0..cols).map(|j| self.0[i][j] + other.0[i][j]).collect()
                    })
                })
                .collect();

            Matrix(threads.into_iter()
                .map(|t| t.join().unwrap())
                .collect())
        })
    }


}