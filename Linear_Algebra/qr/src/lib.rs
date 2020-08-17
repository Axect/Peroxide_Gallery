extern crate peroxide;
use peroxide::fuga::*;

#[allow(non_snake_case)]
pub fn qr(A: &Matrix) -> (Matrix, Matrix) {
    let m = A.row;
    let n = A.col;

    let mut B = A.clone();
    let mut Q = eye(m);
    let sub = if m == n { 1 } else { 0 };
    for i in 0 .. n - sub {
        let mut H = eye(m);
        let hh = gen_householder(&A.col(i).skip(i));
        for j in i .. m {
            for k in i .. m {
                H[(j, k)] = hh[(j-i, k-i)];
            }
        }
        Q = &Q * &H;
        B = &H * &B;
    }
    (Q, B)
}

#[allow(non_snake_case)]
pub fn gen_householder(a: &Vec<f64>) -> Matrix {
    let mut v = a.fmap(|t| t / (a[0] + a.norm(Norm::L2) * a[0].signum()));
    v[0] = 1f64;
    let mut H = eye(a.len());
    let vt: Matrix = v.clone().into();
    H = H - 2f64 / v.dot(&v) * (&vt * &vt.t());
    H
}
