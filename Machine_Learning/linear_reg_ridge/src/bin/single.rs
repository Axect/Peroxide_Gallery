extern crate gaussian;
extern crate peroxide;
use gaussian::*;
use peroxide::fuga::*;

fn main() {
    let sample = gen_sample();
    sample.write_nc("data/data.nc").expect("Can't write data.nc");
    let t: Vec<f64> = sample["y"].to_vec();

    let s = 5f64;
    let lam = 1f64;
    //let w = w_mle(s, t); // Just regression
    let w_reg = w_ml_reg(s, lam, &t);

    let x_draw = seq(1, 100, 0.1);
    let y_draw = x_draw.fmap(|x| y(s, &w_reg, x));

    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x_draw));
    df.push("y", Series::new(y_draw));

    //df.write_nc("reg.nc").expect("Can't write reg.nc");
    df.write_nc("data/single/reg.nc").expect("Can't write reg_lam_1.nc");
}
