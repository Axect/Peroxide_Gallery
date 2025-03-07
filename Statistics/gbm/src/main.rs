use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gbm = GBM {
        x0: 1.0,
        mu: 1.0,
        sigma: 0.5,
        dt: 1e-4,
        seed: 42,
    };

    let steps = 100000;
    let exact = gbm.exact_process(steps);
    let euler = gbm.euler_maruyama(steps);
    let t = linspace(0, steps as f64, steps + 1).mul_s(gbm.dt);
    let drift = t.fmap(|t| gbm.x0 * ((gbm.mu - 0.5 * gbm.sigma.powi(2)) * t).exp());

    let mut df = DataFrame::new(vec![]);
    df.push("t", Series::new(t));
    df.push("Exact", Series::new(exact));
    df.push("Euler", Series::new(euler));
    df.push("Drift", Series::new(drift));
    df.print();
    df.write_parquet("GBM.parquet", CompressionOptions::Snappy)?;

    Ok(())
}

pub struct GBM {
    x0: f64,
    mu: f64,
    sigma: f64,
    dt: f64,
    seed: u64,
}

impl GBM {
    #[allow(non_snake_case)]
    fn exact_process(&self, steps: usize) -> Vec<f64> {
        let t = linspace(0, steps as f64, steps + 1).mul_s(self.dt);
        let mut rng = stdrng_from_seed(self.seed);
        let mut X_vec = vec![self.x0; steps + 1];
        let normal = Normal(0f64, self.dt.sqrt());
        let dW_vec = normal.sample_with_rng(&mut rng, steps);
        let mut W = 0f64;

        for i in 1..steps+1 {
            W += dW_vec[i-1];
            X_vec[i] = self.x0 * ((self.mu - 0.5 * self.sigma.powi(2)) * t[i] + self.sigma * W).exp();
        }

        X_vec
    }

    #[allow(non_snake_case)]
    fn euler_maruyama(&self, steps: usize) -> Vec<f64> {
        let mut rng = stdrng_from_seed(self.seed);
        let mut X_vec = vec![self.x0; steps + 1];
        let normal = Normal(0f64, self.dt.sqrt());
        let dW_vec = normal.sample_with_rng(&mut rng, steps);

        for i in 1..steps + 1 {
            X_vec[i] = X_vec[i-1] + self.mu * X_vec[i-1] * self.dt + self.sigma * X_vec[i-1] * dW_vec[i-1];
        }

        X_vec
    }
}
