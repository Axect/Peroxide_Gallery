#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    let a = ml_matrix("1 2;2 3;3 4");
    let svd = svd(&a);
    svd.u.print();
    svd.b.print();
    svd.v.print();
    ((&svd.u * &svd.b) * svd.v.t()).print();
}

#[derive(Debug, Clone)]
pub struct SVD {
    u: Matrix,
    b: Matrix,
    v: Matrix,
}

fn svd(a: &Matrix) -> SVD {
    let m = a.row;
    let n = a.col;
    let mut b = a.clone();
    let mut u = Matrix::from_index(|i, j| 
        if i == j {
            1f64
        } else {
            0f64
        }
    , (m, n));
    u.print();
    let mut v = eye(n);
    for k in 0 .. n {
        println!("col: {} start", k);
        // q = m * m
        let q = gen_col_householder(k, &b.col(k));
        q.print();
        b = &q * &b;
        u = &u * &q;

        println!("col: {} end", k);
        if k < n-2 {
            let p = gen_row_householder(k+1, &b.row(k));
            println!("row: {}", k);
            b = &b * &p;
            v = &p * &v;
        }
    }

    SVD {
        u,
        b,
        v
    }
}

pub fn householder_vec(x: &Vec<f64>) -> Vec<f64> {
    let m = x.norm(Norm::LInf);
    let x_s = x.fmap(|t| t / m);
    //let x_s = x.clone();
    let mut u = x_s.clone();
    let x_s_norm = x_s.norm(Norm::L2);
    u[0] = x_s[0] + x_s_norm * x_s[0].signum();
    u
}

pub fn householder_mat(u: &Vec<f64>) -> Matrix {
    let u_mat: Matrix = u.into();
    let n = u.len();
    let i = eye(n);
    i - 2f64 * (&u_mat * &u_mat.t()) / u.dot(&u)
}

pub fn gen_col_householder(k: usize, x: &Vec<f64>) -> Matrix {
    let m = x.len();
    let mut q = eye(m);
    // u: m-k+1
    // h: (m-k+1) x (m-k+1)
    let u = householder_vec(&x.skip(k));
    let h = householder_mat(&u);
    for i in 0 .. m-k {
        for j in 0 .. m-k {
            q[(k+i, k+j)] = h[(i, j)];
        }
    }
    q
}

pub fn gen_row_householder(k: usize, x: &Vec<f64>) -> Matrix {
    let q = gen_col_householder(k, x);
    q.t()
}
