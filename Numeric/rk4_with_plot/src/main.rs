use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem = TestODE;
    let ode_solver = BasicODESolver::new(RK4);
    let (t, y) = ode_solver.solve(&problem, (0.0, 10.0), 0.01)?;
    let y = y.into_iter().flatten().collect();

    let mut plt = Plot2D::new();
    plt
        .set_domain(t)
        .insert_image(y)
        .set_xlabel(r"$t$")
        .set_ylabel(r"$y$")
        .set_style(PlotStyle::Nature)
        .tight_layout()
        .set_dpi(600)
        .set_path("plot.png")
        .savefig()?;

    Ok(())
}

struct TestODE;

impl ODEProblem for TestODE {
    fn initial_conditions(&self) -> Vec<f64> {
        vec![1f64]
    }

    fn rhs(&self, t: f64, y: &[f64], dy: &mut [f64]) -> anyhow::Result<()> {
        dy[0] = (5f64 * t.powi(2) - y[0]) / (t + y[0]).exp();
        Ok(())
    }
}
