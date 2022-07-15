use peroxide::fuga::*;

fn main() {
    let x = seq(-2f64, 2f64, 0.2);
    let y = x.fmap(|t| (50f64 * (t-1f64)).tanh() - (50f64 * (t+1f64)).tanh());

    let cs = cubic_hermite_spline(&x, &y, Quadratic);
    let p = |x: f64| {
        if x < -2f64 {
            -2f64
        } else if x > 0f64 {
            0f64
        } else {
            x
        }
    };

    let new_x = linspace(x[0], x[x.len()-1], 1000);
    let y_cs = cs.eval_vec(&new_x);
    let y_cs_cond = cs.eval_vec_with_cond(&new_x, p);

    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x));
    df.push("y", Series::new(y));
    df.push("new_x", Series::new(new_x));
    df.push("y_cs", Series::new(y_cs));
    df.push("y_cs_cond", Series::new(y_cs_cond));

    df.print();

    df.write_nc("data.nc").expect("failed to write data");
}
