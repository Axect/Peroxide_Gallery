use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem = Simple;
    let finder = BisectionMethod { max_iter: 100, tol: 1e-15 };
    let x = finder.find(&problem)?;
    x[0].print();

    Ok(())
}

struct Simple;

impl RootFindingProblem<1, 1, (f64, f64)> for Simple {
    fn initial_guess(&self) -> (f64, f64) {
        (0f64, 5f64)
    }

    fn function(&self, x: Pt<1>) -> anyhow::Result<Pt<1>> {
        Ok([x[0].exp() - (x[0] + 2f64)])
    }
}
