extern crate peroxide;
use peroxide::*;

fn main() {
    // Declare Explicit ODE
    let mut ex_solver = ExplicitODE::new(a);

    let init_state: State<f64> = State::new(
        0.0,        // t=0
        c!(20, 0),  // s=20, v=0
        c!(0, -10)  // v=0,  a=-10
    );

    ex_solver
        .set_initial_condition(init_state)
        .set_method(ExMethod::Euler)
        .set_step_size(1e-6)
        .set_times(3e+6 as usize)
        .set_stop_condition(stop);

    let result = ex_solver.integrate();

    let mut df = DataFrame::with_header(vec!["t", "v", "s"]);
    df["t"] = result.col(0);
    df["v"] = result.col(2);
    df["s"] = result.col(1);

    df.write_nc("data/phys.nc").expect("Can't write phys.nc");
}

/// Acceleration
fn a(st: &mut State<f64>) {
    // x: s, v
    let x = &st.value;
    // dx: v, a
    let dx = &mut st.deriv;
    // Displacement
    dx[0] = x[1];                           // ds/dt = v
    dx[1] = -10f64 + 0.1 * x[1].powi(2);    // dv/dt = a = -10 + 0.1v^2
}

/// Stop condition
///
/// If `s` <= 0, stop!
fn stop(st: &ExplicitODE) -> bool {
    let s = &st.get_state().value[0];
    *s <= 0f64
}
