extern crate peroxide;
extern crate matrixmultiply;
use peroxide::*;
use matrixmultiply::dgemm;

fn main() {
    let a = ml_matrix("1 2 3;4 5 6");
    let b = c!(1, 2, 3);
    let mut c = vec![0f64; 2];
    gemv(1f64, &a, &b, 0f64, &mut c);
    c.print();
}

pub fn gemv(alpha: f64, a: &Matrix, b: &Vec<f64>, beta: f64, c: &mut Vec<f64>) {
    let m = a.row;
    let k = a.col;
    let n = 1usize;
    let (rsa, csa) = match a.shape {
        Row => (a.col as isize, 1isize),
        Col => (1isize, a.row as isize)
    };
    let (rsb, csb) = (1isize, 1isize);
    let (rsc, csc) = (1isize, 1isize);

    unsafe {
        dgemm(
            m,
            k,
            n,
            alpha,
            a.ptr(),
            rsa,
            csa,
            b.as_ptr(),
            rsb,
            csb,
            beta,
            c.as_mut_ptr(),
            rsc,
            csc,
        )
    }
}
