use std::iter::repeat;
use peroxide::fuga::*;
use rayon::prelude::*;
use Kernel::*;

const N: usize = 10000;
const LAMBDA: f64 = 1f64;

fn main() {
    let end_points = repeat(1000).take(N).map(random_walk_end).collect::<Vec<_>>();
    let x_domain = seq(-100, 100, 0.1);
    let kde = |x0: f64| parzen_kde(x0, &end_points, LAMBDA, Epanechnikov);
    let pdf_estimate = x_domain.fmap(kde);

    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x_domain));
    df.push("pdf", Series::new(pdf_estimate));

    df.print();

    df.write_nc("kde.nc").expect("failed to write nc file");
}

fn random_walk_end(n: usize) -> f64 {
    let bern = Bernoulli(0.5);
    let events = bern.sample(n);
    events.into_iter().map(|x| 2f64*(x as f64) - 1f64).sum::<f64>() / 10f64
}

fn gaussian_kernel(x: f64, x0: f64, lambda: f64) -> f64 {
    (- (x - x0).powi(2) / (2f64 * lambda.powi(2))).exp()
}

fn epanechnikov_kernel(x: f64, x0: f64, lambda: f64) -> f64 {
    let t = (x - x0).abs() / lambda;
    if t < 1f64 {
        0.75f64 * (1f64 - t.powi(2))
    } else {
        0f64
    }
}

#[derive(Debug, Copy, Clone)]
enum Kernel {
    Gaussian,
    Epanechnikov,
}

fn parzen_kde(x0: f64, x_vec: &Vec<f64>, lambda: f64, kernel: Kernel) -> f64 {
    let n = x_vec.len();
    match kernel {
        Kernel::Epanechnikov => {
            x_vec.par_iter().fold(|| 0f64, |acc, &x| acc + epanechnikov_kernel(x, x0, lambda)).sum::<f64>() / (n as f64 * lambda)
        },
        Kernel::Gaussian => {
            x_vec.par_iter().fold(|| 0f64, |acc, &x| acc + gaussian_kernel(x, x0, lambda)).sum::<f64>() / (n as f64 * lambda)
        },
    }
}
