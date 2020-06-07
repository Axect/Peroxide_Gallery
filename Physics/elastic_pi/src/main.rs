extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    let m1 = 1f64;
    let m2 = 1e+14;
    let mut p = Phys2D::from_pair((m1, 0f64), (m2, 1f64));
    p.calc().print();
}

struct Phys2D {
    pub m1: f64,
    pub m2: f64,
    pub v1: f64,
    pub v2: f64,
}

impl Phys2D {
    fn from_pair((m1, v1): (f64, f64), (m2, v2): (f64, f64)) -> Self {
        Self {
            m1,
            m2,
            v1,
            v2,
        }
    }

    fn collision_matrix(&self) -> Matrix {
        let m1_plus_m2 = self.m1 + self.m2;
        let x = (self.m2 - self.m1) / m1_plus_m2;
        let y1 = 2f64 * self.m2 / m1_plus_m2;
        let y2 = -2f64 * self.m1 / m1_plus_m2;
        matrix(vec![x, y2, y1, x], 2, 2, Col)
    }

    fn update(&mut self, a: &Matrix) {
        let new_v = a * &vec![self.v1, self.v2];
        self.v1 = new_v[0];
        self.v2 = new_v[1];
    }

    fn calc(&mut self) -> usize {
        let mut n = 0usize;
        let a = self.collision_matrix();
        loop {
            self.update(&a);
            n += 1;
            println!("{:.4}\t{:.4}", self.v1, self.v2);
            if self.v2 < 0f64 && self.v1.abs() < self.v2.abs() {
                break;
            }
        }
        if self.v1 > 0f64 {
            2 * n
        } else {
            2 * n - 1
        }
    }
}
