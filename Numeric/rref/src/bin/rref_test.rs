extern crate peroxide;
extern crate rref;
use peroxide::fuga::*;
use rref::*;

fn main() {
    let a = ml_matrix("1 2 -1 -4; 2 3 -1 -11; -2 0 -3 22");
    a.print();
    rref(&a).print();
}
