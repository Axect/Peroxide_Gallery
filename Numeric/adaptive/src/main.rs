use peroxide::fuga::*;
use quadrature::clenshaw_curtis;
use std::f64::consts::PI;

fn main() {
    //let a = adaptive_trapezoidal(f, (0, PI / 4f64), 1e-15);
    //a.print();
    let b = integrate(f, (0f64, PI/4f64), GaussLegendre(29));
    b.print();

    let c = clenshaw_curtis::integrate(f, 0f64, PI/4f64, 1e-17).integral;
    c.print();
}

fn f(x: f64) -> f64 {
    (3f64 * x).exp() * (2f64 * x).sin()
}

#[allow(non_snake_case)]
fn adaptive_trapezoidal<F: Fn(f64) -> f64 + Copy, T: Into<f64>, S: Into<f64>>(f: F, (a, b): (T, S), tol: f64) -> f64 {
    let mut I = 0f64;
    let mut S: Vec<(f64, f64)> = vec![];
    S.push((a.into(), b.into()));

    while !S.is_empty() {
        let (a, b) = S.pop().unwrap();
        let I_1 = (b - a) / 2f64 * (f(a) + f(b)); // Trapezoidal Rule
        let m = (a + b) / 2f64;
        let I_2 = (b - a) / 4f64 * (f(a) + 2f64 * f(m) + f(b)); // Composite Trapezoidal Rule
        if (I_1 - I_2).abs() < 3f64 * (b - a) * tol {
            I += I_2;
        } else {
            S.push((a, m));
            S.push((m, b));
        }
    }
    I
}

#[allow(non_snake_case)]
fn adaptive_trapezoidal_recur<F: Fn(f64) -> f64 + Copy, T: Into<f64>, S: Into<f64>>(f: F, (a, b): (T, S), tol: f64) -> f64 {
    let mut I = 0f64;
    let (a,b) = (a.into(), b.into());
    let I_1 = (b - a) / 2f64 * (f(a) + f(b)); // Trapezoidal Rule
    let m = (a + b) / 2f64;
    let I_2 = (b - a) / 4f64 * (f(a) + 2f64 * f(m) + f(b)); // Composite Trapezoidal Rule
    if (I_1 - I_2).abs() < 3f64 * (b - a) * tol {
        I += I_2;
    } else {
        I += adaptive_trapezoidal(f, (a, m), tol);
        I += adaptive_trapezoidal(f, (m, b), tol);
    }
    I
}
