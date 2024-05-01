use peroxide::fuga::*;
use core::ops::Range;

fn main() -> anyhow::Result<()> {
    let cubic_b_spline = CubicBSpline::divide((0f64, 1f64), 5);
    let x = linspace(0f64, 1f64, 1000);

    let mut plt = Plot2D::new();
    plt.set_domain(x.clone());

    for basis in &cubic_b_spline.bases {
        plt.insert_image(basis.eval_vec(&x));
    }
    plt
        .insert_image(cubic_b_spline.eval_vec(&x))
        .set_xlabel(r"$x$")
        .set_ylabel(r"$y$")
        .set_style(PlotStyle::Nature)
        .tight_layout()
        .set_dpi(600)
        .set_path("plot.png")
        .savefig()?;

    Ok(())
}


/// Unit Cubic Basis Function
///
/// # Description
/// Unit cubic basis function from Irwin-Hall distribution (n=4).
/// For general interval, we substitute t = 4 * (x - a) / (b - a).
///
/// # Reference
/// [Wikipedia](https://en.wikipedia.org/wiki/Irwin%E2%80%93Hall_distribution#Special_cases)
#[derive(Debug, Copy, Clone)]
pub struct CubicBasis {
    pub x_min: f64,
    pub x_max: f64,
    pub scale: f64,
}

impl CubicBasis {
    pub fn new(x_min: f64, x_max: f64, scale: f64) -> Self {
        Self { x_min, x_max, scale }
    }

    pub fn eval(&self, x: f64) -> f64 {
        let t = 4f64 * (x - self.x_min) / (self.x_max - self.x_min);

        let result = if (0f64..1f64).contains(&t) {
            t.powi(3) / 6f64
        } else if (1f64..2f64).contains(&t) {
            (-3f64 * t.powi(3) + 12f64 * t.powi(2) - 12f64 * t + 4f64) / 6f64
        } else if (2f64..3f64).contains(&t) {
            (3f64 * t.powi(3) - 24f64 * t.powi(2) + 60f64 * t - 44f64) / 6f64
        } else if (3f64..4f64).contains(&t) {
            (4f64 - t).powi(3) / 6f64
        } else {
            0f64
        };

        result * self.scale
    }

    pub fn eval_vec(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|x| self.eval(*x)).collect()
    }
}

/// Uniform Cubic B-Spline basis functions
#[derive(Debug, Clone)]
pub struct CubicBSpline {
    pub ranges: Vec<Range<f64>>,
    pub bases: Vec<CubicBasis>,
}

impl CubicBSpline {
    pub fn new(ranges: Vec<Range<f64>>, bases: Vec<CubicBasis>) -> Self {
        Self { ranges, bases }
    }

    pub fn divide((a, b): (f64, f64), num_bases: usize) -> Self {
        let nodes = linspace(a, b, num_bases + 4);
        let (ranges, bases) = nodes
            .iter()
            .zip(nodes.iter().skip(4))
            .map(|(a, b)| (Range { start: *a, end: *b }, CubicBasis::new(*a, *b, 1f64)))
            .unzip();
        
        Self::new(ranges, bases)
    }

    pub fn eval(&self, x: f64) -> f64 {
        self.ranges.iter()
            .enumerate()
            .filter(|(_, range)| range.contains(&x))
            .fold(0f64, |acc, (i, _)| {
                let basis = &self.bases[i];
                acc + basis.eval(x)
            })
    }

    pub fn eval_vec(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|x| self.eval(*x)).collect()
    }
}
