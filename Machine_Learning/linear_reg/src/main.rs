extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    let sample = gen_sample();
    sample.write_nc("data/data.nc").expect("Can't write data.nc");

    //let x: Vec<f64> = sample["x"].to_vec();
    let t: Vec<f64> = sample["y"].to_vec();

    let s = 1f64;
    let w = w_mle(s, &t);

    let x_draw = seq(1, 100, 0.1);
    let y_draw = x_draw.fmap(|x| y(s, &w, x));

    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x_draw));
    df.push("y", Series::new(y_draw));

    df.write_nc("data/reg.nc").expect("Can't write reg.nc");
}

fn f(x: f64) -> f64 {
    (x / 10f64).sin() + (x / 50f64).powi(2)
}

fn gen_sample() -> DataFrame {
    let mut df = DataFrame::new(vec![]);
    let normal = Normal(0, 1);
    let e = normal.sample(100).fmap(|t| 0.2 * t);
    let x = seq(1, 100, 1);
    let y = x.fmap(f).add_v(&e);

    df.push("x", Series::new(x));
    df.push("y", Series::new(y));
    df
}

fn design_matrix(s: f64) -> Matrix {
    Matrix::from_index(|i, j| phi(j, s, (i+1) as f64), (100, 101))
}

fn phi(j: usize, s: f64, x: f64) -> f64 {
    if j == 0 {
        return 1f64;
    }
    let mu = j as f64;
    (-(x - mu).powi(2) / (2f64 * s.powi(2))).exp()
}

fn phi_vec(s: f64, x: f64) -> Vec<f64> {
    let mut v = vec![0f64; 101];
    for i in 0 .. v.len() {
        v[i] = phi(i, s, x);
    }
    v
}

fn y(s: f64, w: &Vec<f64>, x: f64) -> f64 {
    let phi = phi_vec(s, x);
    w.dot(&phi)
}

fn w_mle(s: f64, t: &Vec<f64>) -> Vec<f64> {
    let phi_mat = design_matrix(s);
    phi_mat.pseudo_inv().apply(t)
}
