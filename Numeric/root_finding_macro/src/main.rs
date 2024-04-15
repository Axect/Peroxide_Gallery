use peroxide::fuga::*;

fn main() -> anyhow::Result<()> {
    let problem = Simple;
    let newton = NewtonMethod { max_iter: 100, tol: 1e-15 };
    let x = newton.find(&problem)?;
    x[0].print();

    Ok(())
}

#[ad_function] // It generates `f_ad: AD -> AD` automatically
fn f(x: f64) -> f64 {
    x.exp() - (x + 2f64)
}

struct Simple;

impl RootFindingProblem<1, 1, f64> for Simple {
    fn initial_guess(&self) -> f64 {
        1f64
    }

    fn function(&self, x: Pt<1>) -> anyhow::Result<Pt<1>> {
        Ok([f(x[0])])
    }

    fn derivative(&self, x: Pt<1>) -> anyhow::Result<Jaco<1, 1>> {
        let ad1 = AD1(x[0], 1f64);
        let dx = f_ad(ad1).dx();
        Ok([[dx]])
    }
}
