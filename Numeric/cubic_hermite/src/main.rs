use peroxide::fuga::*;

fn main() {
    let x = seq(-2f64, 2f64, 0.2);
    let y = x.fmap(|t| (50f64 * (t-1f64)).tanh() - (50f64 * (t+1f64)).tanh());

    let cs = cubic_spline(&x, &y);
    let cs_akima = cubic_hermite_spline(&x, &y, Akima);
    let cs_quad = cubic_hermite_spline(&x, &y, Quadratic);

    let new_x = linspace(x[0], x[x.len()-1], 1000);
    let y_cs = cs.eval_vec(&new_x);
    let y_akima = cs_akima.eval_vec(&new_x);
    let y_quad = cs_quad.eval_vec(&new_x);


    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x));
    df.push("y", Series::new(y));
    df.push("new_x", Series::new(new_x));
    df.push("y_cs", Series::new(y_cs));
    df.push("y_akima", Series::new(y_akima));
    df.push("y_quad", Series::new(y_quad));

    df.print();
    df.write_nc("data.nc").expect("Can't write nc");
}
