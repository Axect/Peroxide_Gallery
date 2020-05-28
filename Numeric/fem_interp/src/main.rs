extern crate peroxide;
use peroxide::*;
use std::f64::consts::PI;

fn main() {
    let mut df = DataFrame::with_header(vec!["h", "int_uh", "e_l2", "e_h1", "u_h2", "u_h1"]);
    let mut hs: Vec<f64> = vec![];
    let mut int_uh: Vec<f64> = vec![];
    let mut e_l2: Vec<f64> = vec![];
    let mut e_h1: Vec<f64> = vec![];
    let mut u_h2: Vec<f64> = vec![];
    let mut u_h1: Vec<f64> = vec![];
    for i in 1 .. 11 {
        let h = 1f64 / 2f64.powi(i);
        let ps = piecewise_1d(u, h);
        hs.push(h);
        int_uh.push(poly_int_sum(&ps, h));
        e_l2.push(measure_error(u, &ps, h, Norm::L2));
        u_h2.push(measure_norm(u, Norm::H2) * 0.01 * h.powi(2));
        e_h1.push(measure_error(u, &ps, h, Norm::H1));
        u_h1.push(measure_norm(u, Norm::H2) * 0.1 * h);
    }
    df["h"] = hs;
    df["int_uh"] = int_uh;
    df["e_l2"] = e_l2;
    df["e_h1"] = e_h1;
    df["u_h2"] = u_h2;
    df["u_h1"] = u_h1;

    df.write_nc("data.nc").expect("Can't write nc");

    integrate(|x: f64| u(hyper_dual(x, 0f64, 0f64)).to_f64(), (0f64, 1f64), GaussLegendre(15)).print();
    (2f64 / PI).print();
}

fn u(x: HyperDual) -> HyperDual {
    (x * PI).sin()
}

fn piecewise_1d<R1, R2>(u: fn(R1) -> R2, h: f64) -> Vec<Polynomial> 
where
    R1: Real,
    R2: Real 
{
    let mut result: Vec<Polynomial> = vec![];
    let x = seq(0, 1, h);
    let y = x.fmap(|t: f64| u(R1::from_f64(t)).to_f64());
    let n = x.len();

    for i in 0 .. n-1 {
        let p1 = lagrange_polynomial(vec![x[i], x[i+1]], vec![y[i], 0f64]);
        let p2 = lagrange_polynomial(vec![x[i], x[i+1]], vec![0f64, y[i+1]]);
        result.push(p1 + p2);
    }

    result
}

fn poly_int(p: &Polynomial, (a, b): (f64, f64)) -> f64 {
    let f = p.integral();
    f.eval(b) - f.eval(a)
}

fn poly_int_sum(ps: &Vec<Polynomial>, h: f64) -> f64 {
    let mut s = 0f64;
    let mut curr_h = 0f64;
    for p in ps {
        s += poly_int(&p, (curr_h, curr_h + h));
        curr_h += h;
    }
    s
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Norm {
    L2,
    H1,
    H2
}

fn measure_error<R1, R2>(u: fn(R1) -> R2, ps: &Vec<Polynomial>, h: f64, norm: Norm) -> f64 
where 
    R1: Real,
    R2: Real
{
    let mut s = 0f64;
    let mut curr_h = 0f64;
    match norm {
        Norm::L2 => {
            for p in ps {
                let f = |x: f64| (u(R1::from_f64(x)).to_f64() - p.eval(x)).powi(2);
                s += integrate(f, (curr_h, curr_h + h), GaussLegendre(15));
                curr_h += h;
            }
            s
        }
        Norm::H1 => {
            for p in ps {
                let f = |x: f64| (u(R1::from_f64(x)).to_f64() - p.eval(x)).powi(2);
                let df = |x: f64| {
                    let dx = dual(x, 1f64);
                    let du = u(R1::from_dual(dx)).to_dual();
                    (du.slope() - p.diff().eval(x)).powi(2)
                };
                s += integrate(f, (curr_h, curr_h + h), GaussLegendre(15));
                s += integrate(df, (curr_h, curr_h + h), GaussLegendre(15));
                curr_h += h;
            }
            s
        }
        Norm::H2 => {
            unimplemented!()
        }
    }
}

fn measure_norm<R1, R2>(u: fn(R1) -> R2, norm: Norm) -> f64 
where
    R1: Real,
    R2: Real,
{
    match norm {
        Norm::L2 => {
            let f = |x: f64| u(R1::from_f64(x)).to_f64().powi(2);
            integrate(f, (0f64, 1f64), GaussLegendre(15))
        }
        Norm::H1 => {
            let df = |x: f64| {
                let dx = dual(x, 1f64);
                let du = u(R1::from_dual(dx)).to_dual();
                du.slope().powi(2)
            };
            let l2 = measure_norm(u, Norm::L2);
            l2 + integrate(df, (0f64, 1f64), GaussLegendre(15))
        }
        Norm::H2 => {
            let ddf = |x: f64| {
                let ddx = hyper_dual(x, 1f64, 0f64);
                let ddu = u(R1::from_hyper_dual(ddx)).to_hyper_dual();
                ddu.accel().powi(2)
            };
            let h1 = measure_norm(u, Norm::H1);
            h1 + integrate(ddf, (0f64, 1f64), GaussLegendre(15))
        }
    }
}
