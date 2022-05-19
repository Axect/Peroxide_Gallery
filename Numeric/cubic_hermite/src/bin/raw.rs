use std::ops::Range;
use std::cmp::{max, min};

use peroxide::fuga::*;

fn main() {
    let x = seq(-2f64, 2f64, 0.2);
    let y = x.fmap(|t| (50f64 * (t-1f64)).tanh() - (50f64 * (t+1f64)).tanh());

    let dydx_1 = akima(&x, &y);
    let dydx_2 = slope_via_quad(&x, &y);

    let cs = cubic_spline(&x, &y);
    let cs_akima = cubic_hermite_spline(&x, &y, &dydx_1);
    let cs_quad = cubic_hermite_spline(&x, &y, &dydx_2);

    let new_x = linspace(x[0], x[x.len()-1], 1000);
    let y_cs = new_x.fmap(|t| cs.eval(t));
    let y_akima = new_x.fmap(|t| eval(&cs_akima, t));
    let y_quad = new_x.fmap(|t| eval(&cs_quad, t));


    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x));
    df.push("y", Series::new(y));
    df.push("akima", Series::new(dydx_1));
    df.push("quad", Series::new(dydx_2));
    df.push("new_x", Series::new(new_x));
    df.push("y_cs", Series::new(y_cs));
    df.push("y_akima", Series::new(y_akima));
    df.push("y_quad", Series::new(y_quad));

    df.print();
    df.write_nc("data.nc").expect("Can't write nc");
}

fn akima(x: &Vec<f64>, y: &Vec<f64>) -> Vec<f64> {
    if x.len() < 3 {
        panic!("Cubic spline need at least 3 nodes");
    }

    let mut m = vec![0f64; x.len()];
    let mut s = vec![0f64; x.len()+3]; // -2, -1, 0, ..., x.len()-1, x.len()

    let l_i = lagrange_polynomial(x[0..3].to_vec(), y[0..3].to_vec());
    let l_f = lagrange_polynomial(x[x.len()-3..].to_vec(), y[y.len()-3..].to_vec());

    let x_i = x[0] - (x[1] - x[0]) / 10f64;
    let x_ii = x_i - (x[1] - x[0]) / 10f64;
    let x_f = x[x.len()-1] + (x[x.len()-1] - x[x.len()-2]) / 10f64;
    let x_ff = x_f + (x[x.len()-1] - x[x.len()-2]) / 10f64;

    let y_i = l_i.eval(x_i);
    let y_ii = l_i.eval(x_ii);
    let y_f = l_f.eval(x_f);
    let y_ff = l_f.eval(x_ff);

    let new_x = concat(&concat(&vec![x_ii, x_i], x), &vec![x_f, x_ff]);
    let new_y = concat(&concat(&vec![y_ii, y_i], y), &vec![y_f, y_ff]);

    for i in 0 .. new_x.len()-1 {
        let dx = new_x[i+1] - new_x[i];
        if dx == 0f64 {
            panic!("x nodes should be different!");
        }
        s[i] = (new_y[i+1] - new_y[i]) / dx;
    }
    
    for i in 0 .. x.len() {
        let j = i+2;
        let ds_f = (s[j+1] - s[j]).abs();
        let ds_i = (s[j-1] - s[j-2]).abs();

        m[i] = if ds_f == 0f64 && ds_i == 0f64 {
            (s[j-1] + s[j]) / 2f64
        } else {
            (ds_f * s[j-1] + ds_i * s[j]) / (ds_f + ds_i)
        };
    }
    m
}

fn slope_via_quad(x: &Vec<f64>, y: &Vec<f64>) -> Vec<f64> {
    let mut m = vec![0f64; x.len()];
    let q_i = lagrange_polynomial(x[0..3].to_vec(), y[0..3].to_vec());
    let q_f = lagrange_polynomial(x[x.len()-3..].to_vec(), y[y.len()-3..].to_vec());

    m[0] = q_i.derivative().eval(x[0]);
    m[x.len()-1] = q_f.derivative().eval(x[x.len()-1]);

    for i in 1 .. x.len()-1 {
        let q = lagrange_polynomial(x[i-1..i+2].to_vec(), y[i-1..i+2].to_vec());
        m[i] = q.derivative().eval(x[i]);
    }

    m
}

fn cubic_hermite_spline(x: &Vec<f64>, y: &Vec<f64>, m: &Vec<f64>) -> Vec<(Range<f64>, Polynomial)> {
    let mut r = vec![Range::default(); x.len()-1];
    let mut u = vec![Polynomial::default(); x.len()-1];

    for i in 0 .. x.len()-1 {
        let a_i = y[i];
        let b_i = m[i];
        let dx = x[i+1] - x[i];
        let dy = y[i+1] - y[i];
        let c_i = (3f64 * dy/dx - 2f64*m[i] - m[i+1]) / dx;
        let d_i = (m[i] + m[i+1] - 2f64 * dy/dx) / dx.powi(2);
        
        let p = Polynomial::new(vec![1f64, -x[i]]);

        r[i] = Range { start: x[i], end: x[i+1] };
        u[i] = p.powi(3) * d_i + p.powi(2) * c_i + p.clone() * b_i + a_i;
    }


    //CubicSpline::from(r.into_iter().zip(u.into_iter()).collect::<Vec<(Range<f64>, Polynomial)>>())
    r.into_iter().zip(u.into_iter()).collect()
}

fn eval(cs: &Vec<(Range<f64>, Polynomial)>, x: f64) -> f64 {
    let index = match cs.binary_search_by(|(range, _)| {
        if range.contains(&x) {
            core::cmp::Ordering::Equal
        } else if x < range.start {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Less
        }
    }) {
        Ok(index) => index,
        Err(index) => max(0, min(index, cs.len() - 1)),
    };

    cs[index].1.eval(x)
}
