use peroxide::fuga::*;

fn main() -> Result<(), RootError> {
    let x = newton(f_ad, 1f64, 100, 1e-15)?;
    x.print();

    Ok(())
}

#[ad_function] // It generates `f_ad: AD -> AD` automatically
fn f(x: f64) -> f64 {
    x.exp() - (x + 2f64)
}
