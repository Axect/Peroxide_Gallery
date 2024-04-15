use peroxide::fuga::*;
use radient::prelude::*;

fn main() -> anyhow::Result<()> {
    let problem = Simple;
    let newton = NewtonMethod { max_iter: 100, tol: 1e-15 };
    let x = newton.find(&problem)?;
    x[0].print();

    Ok(())
}

fn f(x: f64) -> f64 {
    x.exp() - (x + 2f64)
}

fn f_expr(x: &[Expr]) -> Expr {
    let x = &x[0];
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
        let (_, grad) = gradient(f_expr, &x);
        Ok([[grad[0]]])
    }
}
