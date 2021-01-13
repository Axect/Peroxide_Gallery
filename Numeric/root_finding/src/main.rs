extern crate peroxide;
use peroxide::fuga::*;

fn main() -> Result<(), RootError> {
    let x = bisection(f, (0f64, 5f64), 100, 1e-15)?;
    x.print();

    Ok(())
}

fn f(x: AD) -> AD {
    x.exp() - (x + 2f64)
}
