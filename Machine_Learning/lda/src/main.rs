#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    // Generate 2D Random Data
    let x1 = Normal(3,1).sample(150);
    let y1 = Normal(1,3).sample(150);
    let x2 = Normal(-3,1).sample(150);
    let y2 = Normal(-1,3).sample(150);

    // Data for Least square
    // x  : 300 x 1 (Vector)
    // y  : 300 x 1 (Vector)
    // z  : 300 x 3 (Matrix) - Input Data
    // t1 : 300 x 1 (Vector)
    // t2 : 300 x 1 (Vector)
    // t  : 300 x 2 (Matrix) - One hot encoding
    let x = concat(&x1, &x2);
    let y = concat(&y1, &y2);
    let z = hstack!(vec![1f64; 300], x, y);
    let t1 = concat(&vec![1f64; 150], &vec![0f64; 150]);
    let t2 = concat(&vec![0f64; 150], &vec![1f64; 150]);
    let t = hstack!(t1, t2);


    // Weight computation
    // w : 3 x 2 (Matrix)
    let w = weight_ls(&z, &t);
    w.print();

    // Test Classification
    let l1 = least_square(&w, &c!(3, 1));
    l1.print();
    classifier(&l1).print();
    let l2 = least_square(&w, &c!(-3, 1));
    l2.print();
    classifier(&l2).print();
    
    // To draw decision boundary
    let domain = seq(-5, 5, 0.01);
    let b1 = boundary_1(&w, domain.clone());
    let b2 = boundary_2(&w, domain.clone());

    // Fisher
    let g1 = hstack!(x1.clone(), y1.clone());
    let g2 = hstack!(x2.clone(), y2.clone());
    let m1 = g1.mean();
    let m2 = g2.mean();
    let m = m1.add_v(&m2).div_s(0.5);
    m.print();
    let s = g1.cov() + g2.cov();
    s.print();
    s.inv().print();
    let mut w_fisher = weight_fisher(&s, &m1, &m2);
    w_fisher = w_fisher.normalize(Norm::L2);
    w_fisher.print();

    let w_g1 = &w_fisher * &g1.t();
    let w_g2 = &w_fisher * &g2.t();
    let w_0 = w_fisher.dot(&m);

    let mut df = DataFrame::new(vec![]);
    df.push("x1", Series::new(x1));
    df.push("y1", Series::new(y1));
    df.push("x2",Series::new(x2));
    df.push("y2",Series::new(y2));
    df.push("d",Series::new(domain.clone()));
    df.push("b1",Series::new(b1));
    df.push("b2",Series::new(b2));
    df.push("bf",Series::new(boundary_fisher(&w_fisher, &domain, &m)));
    df.push("r1",Series::new(w_g1));
    df.push("r2",Series::new(w_g2));

    df.print();
    df.write_nc("data/lda.nc").expect("Can't write least_square");
}

/// Weight
///
/// # Description
///
/// * x: 300 x 3 (Matrix)
/// * t: 300 x 2 (Matrix)
/// * w:   3 x 2 (Matrix)
fn weight_ls(x: &Matrix, t: &Matrix) -> Matrix {
    let x_dagger = x.pseudo_inv();
    &x_dagger * t
}

/// Least square fitting (Single element)
///
/// # Description
/// * w: 3 x 2 (Matrix)
/// * x: 2 x 1 (Vector)
/// * z: 3 x 1 (Vector)
/// * y: 2 x 1 (Vector)
fn least_square(w: &Matrix, x: &Vec<f64>) -> Vec<f64> {
    let mut z = vec![1f64];
    z.extend(x);
    let y = &z * w;
    y
}

/// Least square fitting (Vectorized)
///
/// # Description
/// * n: # of domain (usize)
/// * w: 3 x 2 (Matrix)
/// * x: 2 x n (Matrix)
/// * z: 3 x n (Matrix)
/// * y: 2 x n (Matrix)
fn least_square_map(w: &Matrix, x: &Matrix) -> Matrix {
    let mut z = zeros(3, x.col);
    z.subs_row(1, &vec![1f64; x.col]);
    z.subs_row(2, &x.row(1));
    z.subs_row(3, &x.row(2));

    let y = w.t() * z;
    y
}

/// Which group?
fn classifier(y: &Vec<f64>) -> usize {
    y.arg_max() + 1
}

/// Decision Boundary 1
fn boundary_1(w: &Matrix, x: Vec<f64>) -> Vec<f64> {
    ((x.ox() * (-w[(1, 0)]) - w[(0, 0)] + 0.5f64) / w[(2, 0)]).red()
}

/// Decision Boundary 2
fn boundary_2(w: &Matrix, x: Vec<f64>) -> Vec<f64> {
    ((x.ox() * (-w[(1, 1)]) - w[(0, 1)] + 0.5f64) / w[(2, 1)]).red()
}

/// Fisher's LDA
fn weight_fisher(s_w: &Matrix, m1: &Vec<f64>, m2: &Vec<f64>) -> Vec<f64> {
    s_w.inv() * m2.sub_v(&m1)
} 

/// Boundary of Fisher's LDA
fn boundary_fisher(w: &Vec<f64>, x: &Vec<f64>, m: &Vec<f64>) -> Vec<f64> {
    x.sub_s(m[0]).mul_s(-w[0] / w[1]).add_s(m[1])
    //(x.ox() * (-w[0] / w[1])).red()
}
