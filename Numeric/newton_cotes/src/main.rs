extern crate peroxide;
use peroxide::*;
use std::f64::consts::PI;

fn main() {
    let x2 = |x: f64| x.powi(2);   
    let sin = |x: f64| x.sin();
    let exp = |x: f64| x.exp();

    newton_cotes_quadrature(x2, 2, (0f64, 1f64)).print();
    newton_cotes_quadrature(sin, 11, (0f64, PI)).print();
    newton_cotes_quadrature(exp, 11, (0f64, 1f64)).print();

    gauss_legendre_quadrature(x2, 2, (0f64, 1f64)).print();
    gauss_legendre_quadrature(sin, 11, (0f64, PI)).print();
    gauss_legendre_quadrature(exp, 11, (0f64, 1f64)).print();
}

fn newton_cotes_quadrature<F>(f: F, n: usize, (a, b): (f64, f64)) -> f64 
where F: Fn(f64) -> f64 
{
    let h = (b - a) / (n as f64);
    let node_x = seq(a, b, h);
    let node_y = node_x.fmap(|x| f(x));
    let p = lagrange_polynomial(node_x, node_y);
    let q = p.integral();

    q.eval(b) - q.eval(a)
}
