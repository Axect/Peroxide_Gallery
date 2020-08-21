#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    // Generate Data
    let x = seq(1, 10, 1);
    let eps_true = rnorm!(10, 0, 1);
    let y = x.fmap(|t| 2f64 * t + 1f64).add_v(&eps_true);
    let data = cbind(x.into(), y.into());

    // Generate Normal error (eps ~ N(0, 1^2))
    let eps_guess = rnorm!(10, 0, 1);

    // Optimize with new error
    let mut opt = Optimizer::new(data, |x, w| linear_regression(x, w, &eps_guess));
    let p = opt.set_init_param(c!(1, 1))
        .set_max_iter(50)
        .set_method(LevenbergMarquardt)
        .optimize();
    p.print();
    opt.get_error().print();
}

// Linear regression with error: y_i = w_0 + w_1 * x_i + epsilon_i
fn linear_regression(x: &Vec<f64>, w: Vec<Number>, epsilon: &Vec<f64>) -> Option<Vec<Number>> {
    Some(
        x.iter()
            .map(|&t| Number::from_f64(t))
            .zip(epsilon.iter().map(|&t| Number::from_f64(t)))
            .map(|(t, eps)| w[0] + w[1] * t + eps)
            .collect()
    )
}
