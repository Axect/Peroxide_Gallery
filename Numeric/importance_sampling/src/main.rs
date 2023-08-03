use peroxide::fuga::*;
use std::time::Instant;


fn main() {
    let a = 0f64;
    let b = 2f64;

    // 0. True
    let true_value = f_integrate_true(b) - f_integrate_true(a);

    // 1. G7K15R
    let time_g7k15r = Instant::now();
    let g7k15r = integrate(f, (a, b), G7K15R(1e-5, 20));
    let time_g7k15r = time_g7k15r.elapsed().as_millis();
    let error_g7k15r = (g7k15r - true_value).abs() / true_value.abs() * 100f64;

    // 2. Importance Sampling
    // 2.1 Uniform
    let time_uniform = Instant::now();
    let uniform = importance_sampling_uniform(f, (a, b), 100_0000);
    let time_uniform = time_uniform.elapsed().as_millis();
    let error_uniform = (uniform - true_value).abs() / true_value.abs() * 100f64;

    // 2.2 Weighted Uniform
    let time_weighted = Instant::now();
    let weighted = importance_sampling_weighted(f, (a, b), 100_0000);
    let time_weighted = time_weighted.elapsed().as_millis();
    let error_weighted = (weighted - true_value).abs() / true_value.abs() * 100f64;

    let mut df = DataFrame::new(vec![]);
    df.push("Method", Series::new(
        vec![
            "G7K15R".to_string(),
            "Uniform".to_string(),
            "Weighted".to_string()
        ]
    ));
    df.push("Time", Series::new(
        vec![
            time_g7k15r as u64,
            time_uniform as u64,
            time_weighted as u64,
        ]
    ));
    df.push("Error", Series::new(
        vec![
            error_g7k15r,
            error_uniform,
            error_weighted,
        ]
    ));
    df.push("Value", Series::new(
        vec![
            g7k15r,
            uniform,
            weighted,
        ]
    ));

    df.print();
}

fn f(x: f64) -> f64 {
    (-x.powi(2)).exp() * x.sin() / (1f64 + x.powi(2))
}

fn f_integrate_true(x: f64) -> f64 {
    if x == 2f64 {
        0.26631160
    } else {
        0f64
    }
}

fn importance_sampling_uniform<F>(f: F, (a, b): (f64, f64), n: usize) -> f64 
where F: Fn(f64) -> f64 {
    let uniform = Uniform(a, b);
    let samples = uniform.sample(n);
    samples.into_iter().fold(0f64, |acc, x| acc + f(x)) * (b - a) / n as f64
}

fn importance_sampling_weighted<F>(f: F, (a, b): (f64, f64), n: usize) -> f64
where F: Fn(f64) -> f64 + Copy {
    let weighted = WeightedUniform::from_max_pool_1d(f, (a, b), 1000, 1e-4);
    let samples = weighted.sample(n);
    samples.into_iter().fold(0f64, |acc, x| {
        acc + f(x) / weighted.pdf(x)
    }) / n as f64
}
