extern crate peroxide;
use peroxide::*;

pub trait Triangular {
    fn back_subs(&self, b: &Vec<f64>) -> Vec<f64>;
    fn forward_subs(&self, b: &Vec<f64>) -> Vec<f64>;
}

impl Triangular for Matrix {
    fn back_subs(&self, b: &Vec<f64>) -> Vec<f64> {
        let n = self.col;
        let mut y = vec![0f64; n];
        y[n-1] = b[n-1] / self[(n-1, n-1)];
        for i in (0 .. n - 1).rev() {
            let mut s = 0f64;
            for j in i+1 .. n {
                s += self[(i, j)] * y[j];
            }
            y[i] = 1f64 / self[(i, i)] * (b[i] - s);
        }
        y
    }

    fn forward_subs(&self, b: &Vec<f64>) -> Vec<f64> {
        let n = self.col;
        let mut y = vec![0f64; n];
        y[0] = b[0] / self[(0, 0)];
        for i in 1 .. n {
            let mut s = 0f64;
            for j in 0 .. i {
                s += self[(i, j)] * y[j];
            }
            y[i] = 1f64 / self[(i, i)] * (b[i] - s);
        }
        y
    }
}
