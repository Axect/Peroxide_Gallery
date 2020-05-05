extern crate peroxide;
extern crate triangular;
use peroxide::*;
use triangular::*;

fn main() {
    let a = ml_matrix("2 1 -1;-3 -1 2;-2 1 2");
    let mut b = c!(8, -11, -3);
    // Complete Pivoting LU decomposition
    let lu = a.lu().unwrap();
    // PAQ = LU
    let (p, q, l, u) = lu.extract();

    // Lz = Pb
    swap_with_perm(&p, &mut b);
    let z = l.forward_subs(&b);

    // Uy = z
    let mut y = u.back_subs(&z);
    
    // y' = Qy
    swap_with_perm(&q, &mut y);
    y.print();

    (a * y).print();
}

// Complete Pivoting
fn swap_with_perm(p: &Vec<(usize, usize)>, b: &mut Vec<f64>) {
    for (i, j) in p.iter() {
        b.swap(*i, *j);
    }
}
