extern crate peroxide;
extern crate qr;
use peroxide::fuga::*;
//use qr::*;

fn main() {
    let b = seq(1, 10, 1);
    let q = gen_householder(&b.skip(5)); 
    q.print();
    (&q * &b).print();
}
