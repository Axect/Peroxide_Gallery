extern crate peroxide;
use peroxide::*;

pub fn f(x: f64) -> f64 {
    (x / 10f64).sin() + (x / 50f64).powi(2)
}

pub fn gen_sample() -> DataFrame {
    let mut df = DataFrame::with_header(vec!["x", "y"]);
    let normal = Normal(0, 1);
    let e = normal.sample(100).fmap(|t| 0.2 * t);
    let x = seq(1, 100, 1);
    let y = x.fmap(f).add(&e);
    df["x"] = x;
    df["y"] = y;
    df
}

pub fn phi(j: usize, s: f64, x: f64) -> f64 {
    if j == 0 {
        return 1f64;
    }
    let mu = j as f64;
    (-(x - mu).powi(2) / s).exp()
}

pub fn phi_vec(s: f64, x: f64) -> Vec<f64> {
    let mut v = vec![0f64; 101];
    for i in 0 .. v.len() {
        v[i] = phi(i, s, x);
    }
    v
}

pub fn design_matrix(s: f64) -> Matrix {
    Matrix::from_index(|i, j| phi(j, s, (i+1) as f64), (100, 101))
}

pub fn w_mle(s: f64, t: &Vec<f64>) -> Matrix {
    let phi_mat = design_matrix(s);
    phi_mat.pseudo_inv().unwrap() * t.to_matrix()
}

pub fn y(s: f64, w: &Matrix, x: f64) -> f64 {
    let phi = phi_vec(s, x);
    (w.t() * phi)[(0, 0)]
}


pub fn w_ml_reg(s: f64, lam: f64, t: &Vec<f64>) -> Matrix {
    let phi_mat = design_matrix(s);
    let lamda_eye = lam * eye(101);
    let pt = phi_mat.t();
    (&lamda_eye + &(&pt * &phi_mat)).inv().unwrap() * pt * t.to_matrix()
}
