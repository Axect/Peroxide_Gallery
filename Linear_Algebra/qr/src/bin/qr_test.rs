extern crate peroxide;
extern crate qr;
use peroxide::*;
use qr::*;

fn main() {
    let a = ml_matrix("12 -51 4; 6 167 -68; -4 24 -41");
    let (q, r) = qr(&a);

    a.print();
    q.print();
    r.print();
    (&q * &r).print();
}
