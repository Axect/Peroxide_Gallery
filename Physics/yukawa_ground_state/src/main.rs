#[macro_use]
extern crate peroxide;
extern crate natural_unit;
use peroxide::fuga::*;
use peroxide::numerical::newton::newton;
use natural_unit::CONSTANT_CGS;

fn main() {
    // =========================================================================
    // Newton's Method
    // =========================================================================
    let r_init = c!(1e-12);
    let r0_vec = seq(10, 100, 0.1);
    let mut r_vec = vec![0f64; r0_vec.len()];
    for (i, r0) in r0_vec.iter().enumerate() {
        let func = |v: &Vec<AD>| { f(*r0, v) };
        let sol = newton(r_init.clone(), func, 1e-15);
        r_vec[i] = sol[0];
    }

    // =========================================================================
    // Taylor Expansion
    // =========================================================================
    let taylor = r0_vec.fmap(|k: f64| 0.5 - (-1f64 / k).exp());

    // =========================================================================
    // Write to netcdf
    // =========================================================================
    let mut df = DataFrame::new(vec![]);
    df.push("r0", Series::new(r0_vec));
    df.push("E", Series::new(taylor));
    df.push("EN", Series::new(E0_vec(&r_vec)));

    df.print();

    df.write_nc("data/newton.nc").expect("Can't write nc");
}

fn f(r0: f64, v: &Vec<AD>) -> Vec<AD> {
    let mch = m_e() * CONSTANT_CGS.c / CONSTANT_CGS.hbar;
    vec![(- mch * v[0] / r0).exp() * (1f64 + mch * v[0] / r0) * v[0] - CONSTANT_CGS.hbar / (m_e() * CONSTANT_CGS.c)]
}

fn m_e() -> f64 {
    CONSTANT_CGS.m_u * 5.489e-4
}

#[allow(non_snake_case)]
fn E0_vec(r_vec: &Vec<f64>) -> Vec<f64> {
    let r0_vec = seq(10, 100, 0.1);
    let mut result = vec![0f64; r_vec.len()];
    let hmc = CONSTANT_CGS.hbar / (m_e() * CONSTANT_CGS.c);
    for (i, r) in r_vec.iter().enumerate() {
        let r0 = r0_vec[i];
        result[i] = hmc * (0.5 * hmc / r.powi(2) - 1f64 / r * (-r/(hmc * r0)).exp());
    }
    result
}
