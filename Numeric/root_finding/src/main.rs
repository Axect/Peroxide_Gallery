extern crate peroxide;
use peroxide::fuga::*;

fn main() -> Result<(), RootError> {
    let x = bisection(f, (0f64, 5f64), 100, 1e-15)?;
    x.print();

    Ok(())
}

fn f<T: AD>(x: T) -> T {
    x.exp() - (x + 2f64)
}
