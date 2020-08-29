extern crate peroxide;
use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn Error>> {
    // =========================================
    //  Declare ODE
    // =========================================
    let mut ex_test = ExplicitODE::new(butterfly);

    let init_state: State<f64> = State::new(
        0.0,
        vec![10.0, 1.0, 1.0],
        vec![0.0, 0.0, 0.0],
    );

    ex_test
        .set_initial_condition(init_state)
        .set_method(ExMethod::Euler)
        .set_step_size(0.01f64)
        .set_times(10000);

    let mut ex_test2 = ex_test.clone();
    ex_test2.set_method(ExMethod::RK4);

    // =========================================
    //  Save results
    // =========================================
    let results = ex_test.integrate();
    let results2 = ex_test2.integrate();

    let mut df_euler = DataFrame::from_matrix(results);
    df_euler.set_header(vec!["t", "x", "y", "z"]);
    df_euler.print();

    let mut df_rk4 = DataFrame::from_matrix(results2);
    df_rk4.set_header(vec!["t", "x", "y", "z"]);
    df_rk4.print();

    df_euler.write_nc("data/euler.nc")?;
    df_rk4.write_nc("data/rk4.nc")?;

    Ok(())
}

fn butterfly(st: &mut State<f64>, _: &NoEnv) {
    let x = &st.value;
    let dx = &mut st.deriv;
    dx[0] = 10f64 * (x[1] - x[0]);
    dx[1] = 28f64 * x[0] - x[1] - x[0] * x[2];
    dx[2] = -8f64/3f64 * x[2] + x[0] * x[1];
}
