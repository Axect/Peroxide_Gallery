#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    let init_state = State::<f64>::new(0f64, c!(1), c!(0));

    let mut ode_solver = ExplicitODE::new(test_fn);

    ode_solver
        .set_method(ExMethod::RK4)
        .set_initial_condition(init_state)
        .set_step_size(0.01)
        .set_stop_condition(stop)        // Add stop condition
        .set_times(1000);

    let result = ode_solver.integrate();

    if ode_solver.has_stopped() {
        println!("It reached to stop condition");
    }

    let x = result.col(0);
    let y = result.col(1);

    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x));
    df.push("y", Series::new(y));
    df.print();

    df.write_nc("data/data.nc").expect("Cannot write file");
}

fn test_fn(st: &mut State<f64>, _: &NoEnv) {
    let x = st.param;
    let y = &st.value;
    let dy = &mut st.deriv;
    dy[0] = (5f64 * x.powi(2) - y[0]) / (x + y[0]).exp();
}

fn stop<E: Environment>(st: &ExplicitODE<E>) -> bool {
    let y = &st.get_state().value[0];
    (*y - 2.4).abs() < 0.01
}
