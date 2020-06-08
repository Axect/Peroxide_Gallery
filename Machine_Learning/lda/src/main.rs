#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    // Generate 2D Random Data
    let x1 = Normal(1,1).sample(150);
    let y1 = Normal(0,3).sample(150);
    let x2 = Normal(-3,1).sample(150);
    let y2 = Normal(-1,3).sample(150);

    // Data for Least square
    // x  : 300 x 1 (Vector)
    // y  : 300 x 1 (Vector)
    // z  : 300 x 3 (Matrix) - Input Data
    // t1 : 300 x 1 (Vector)
    // t2 : 300 x 1 (Vector)
    // t  : 300 x 2 (Matrix) - One hot encoding
    let x = concat(x1.clone(), x2.clone());
    let y = concat(y1.clone(), y2.clone());
    let z = hstack!(vec![1f64; 300], x, y);
    let t1 = concat(vec![1f64; 150], vec![0f64; 150]);
    let t2 = concat(vec![0f64; 150], vec![1f64; 150]);
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
    let m1_x = x1.mean();
    let m1_y = y1.mean();
    let m2_x = x2.mean();
    let m2_y = y2.mean();
    let m1 = c!(m1_x, m1_y);
    let m2 = c!(m2_x, m2_y);
    let m = m1.add_vec(&m2);
    let mut s1 = zeros(2, 2);
    s1[(0, 0)] = x1.var();
    s1[(0, 1)] = cov(&x1, &y1);
    s1[(1, 0)] = s1[(0, 1)];
    s1[(1, 1)] = y1.var();
    let mut s2 = zeros(2, 2);
    s2[(0, 0)] = x2.var();
    s2[(0, 1)] = cov(&x2, &y2);
    s2[(1, 0)] = s2[(0, 1)];
    s2[(1, 1)] = y2.var();
    let s = s1 + s2;
    let mut w_fisher = weight_fisher(&s, m1, m2);
    w_fisher = w_fisher.normalize(Norm::L2);
    w_fisher.print();

    let mut df = DataFrame::with_header(vec!["x1", "y1", "x2", "y2", "d", "b1", "b2", "bf"]);
    df["x1"] = x1;
    df["y1"] = y1;
    df["x2"] = x2;
    df["y2"] = y2;
    df["d"] = domain.clone();
    df["b1"] = b1;
    df["b2"] = b2;
    df["bf"] = boundary_fisher(&w_fisher, domain, &m);

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
    let x_dagger = x.pseudo_inv().unwrap();
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
fn weight_fisher(s_w: &Matrix, m1: Vec<f64>, m2: Vec<f64>) -> Vec<f64> {
    s_w.inv().unwrap() * (m2.ox() - m1.ox()).red()
} 

/// Boundary of Fisher's LDA
fn boundary_fisher(w: &Vec<f64>, x: Vec<f64>, m: &Vec<f64>) -> Vec<f64> {
    //((x.ox() - m[0]) * (-w[0] / w[1]) + m[1]).red()
    (x.ox() * (-w[0] / w[1])).red()
}
