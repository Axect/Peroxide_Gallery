use peroxide::fuga::*;

//fn main() {
//    let init_state = State::<f64>::new(0f64, c!(1), c!(0));
//
//    let mut ode_solver = ExplicitODE::new(test_fn);
//
//    ode_solver
//        .set_method(ExMethod::RK4)
//        .set_initial_condition(init_state)
//        .set_step_size(0.01)
//        .set_stop_condition(stop)        // Add stop condition
//        .set_times(1000);
//
//    let result = ode_solver.integrate();
//
//    let x = result.col(0);
//    let y = result.col(1);
//
//    let mut plt = Plot2D::new();
//    plt.set_domain(x)
//        .insert_image(y)
//        .set_title("Test Figure")
//        .set_fig_size((10, 6))
//        .set_dpi(300)
//        .set_legend(vec!["RK4"])
//        .set_path("plot.png");
//
//    plt.savefig().expect("Can't draw plot");
//}
//
//fn test_fn(st: &mut State<f64>) {
//    let x = st.param;
//    let y = &st.value;
//    let dy = &mut st.deriv;
//    dy[0] = (5f64 * x.powi(2) - y[0]) / (x + y[0]).exp();
//}
//
//fn stop(st: &ExplicitODE) -> bool {
//    let y = &st.get_state().value[0];
//    (*y - 2.4).abs() < 0.01
//}

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
