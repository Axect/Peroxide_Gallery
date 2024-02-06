use peroxide::fuga::*;
use std::f64::consts::PI;
use rayon::prelude::*;

fn main() {
    let t = seq(0.0, 0.5, 0.001);
    let omega = 10f64 * 2f64 * PI;
    let x = t.fmap(|t| {
        (1 .. 6).fold(0f64, |sum, i| {
            let if64 = i as f64;
            sum + if64 * (if64 * omega * t).cos()
        })
    });

    let y = dct(&x);
    let x_hat = idct(&y);

    let mut df = DataFrame::new(vec![]);
    df.push("t", Series::new(t));
    df.push("x", Series::new(x));
    df.push("y", Series::new(y));
    df.push("x_hat", Series::new(x_hat));

    df.print();

    df.write_parquet("dct.parquet", CompressionOptions::Uncompressed).unwrap();
}

// Type-II Discrete Cosine Transform
fn dct(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    (0 .. n)
        .into_par_iter()
        .map(|k| {
            x.iter().enumerate().fold(0.0, |sum, (i, x_i)| {
                sum + x_i * (PI * (i as f64 + 0.5) * k as f64 / n as f64).cos()
            }) * 2f64
        })
        .collect()
}

// Type-II inverse Discrete Cosine Transform
fn idct(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let mut beta = vec![1f64; n];
    beta[0] = 0.5;

    (0 .. n)
        .into_par_iter()
        .map(|k| {
            x.iter().zip(beta.iter()).enumerate().fold(0.0, |sum, (i, (x_i, beta_i))| {
                sum + x_i * beta_i * (PI * (i as f64 + 0.5) * k as f64 / n as f64).cos()
            }) / n as f64
        })
        .collect()
}
