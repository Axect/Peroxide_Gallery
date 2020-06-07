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

    least_square(&w, &c!(3, 1)).print();
    least_square(&w, &c!(-3, 1)).print();
    
    // To draw decision boundary
    //let domain = seq(-5, 5, 0.01);
    //let fitted = least_square(&w, &domain);

    //let mut df = DataFrame::with_header(vec!["x1", "y1", "x2", "y2", "domain", "fitted"]);
    //df["x1"] = x1;
    //df["y1"] = y1;
    //df["x2"] = x2;
    //df["y2"] = y2;
    //df["domain"] = domain;
    //df["fitted"] = fitted;

    //df.print();
    //df.write_nc("data/least_square.nc").expect("Can't write least_square");
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

