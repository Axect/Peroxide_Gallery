use peroxide::fuga::*;
//
//fn main() {
//    let x = seq(0, 10, 1);
//    let y = x.iter().enumerate().map(|(i, &t)| t.powi(5-i as i32)).collect::<Vec<f64>>();
//
//    let c = CubicSpline::from_nodes(x, y);
//
//    let init_state = State::<f64>::new(0f64, c!(1), c!(0));
//
//    let mut ode_solver = ExplicitODE::new(test_fn);
//
//    ode_solver
//        .set_method(ExMethod::RK4)
//        .set_initial_condition(init_state)
//        .set_step_size(0.01)
//        .set_times(1000)
//        .set_env(c);
//    
//    let result = ode_solver.integrate();
//
//    let mut df = DataFrame::from_matrix(result);
//    df.set_header(vec!["x", "y"]);
//    df.print();
//    df.write_nc("data/data.nc").expect("Can't write ncfile");
//}
//
//fn test_fn(st: &mut State<f64>, env: &CubicSpline) {
//    let x = st.param;
//    let dy = &mut st.deriv;
//    dy[0] = env.eval(x);
//}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = seq(0, 10, 1);
    let y = x.iter().enumerate().map(|(i, &t)| t.powi(5-i as i32)).collect::<Vec<f64>>();

    let cs = cubic_hermite_spline(&x, &y, Quadratic)?;

    let problem = TestODE { cs };

    let ode_solver = BasicODESolver::new(RK4);
    let (x_ode, y_ode) = ode_solver.solve(&problem, (0.0, 10.0), 0.01)?; 
    let y_ode = y_ode.into_iter().flatten().collect();

    let mut plt = Plot2D::new();
    plt
        .set_domain(x_ode)
        .insert_image(y_ode)
        .set_xlabel(r"$x$")
        .set_ylabel(r"$y$")
        .set_style(PlotStyle::Nature)
        .tight_layout()
        .set_dpi(600)
        .set_path("plot.png")
        .savefig()?;


    Ok(())
}

struct TestODE {
    cs: CubicHermiteSpline
}

impl ODEProblem for TestODE {
    fn initial_conditions(&self) -> Vec<f64> {
        vec![1f64]
    }

    fn rhs(&self, t: f64, _y: &[f64], dy: &mut [f64]) -> anyhow::Result<()> {
        dy[0] = self.cs.eval(t);
        Ok(())
    }
}
