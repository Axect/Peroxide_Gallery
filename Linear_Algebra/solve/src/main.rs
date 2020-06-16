#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    let a = ml_matrix("1 -2 1;0 2 2;-2 4 2");
    let b = c!(1,4,2);

    a.solve(&b, LU).print();    // [2, 1, 1]
    a.solve(&b, WAZ).print();   // [2, 1, 1]
}
