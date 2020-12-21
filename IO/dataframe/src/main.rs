#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn Error>> {
    let x = seq(-1, 1, 0.01);
    let y = x.fmap(|t| t.powi(2));

    // Construct DataFrame
    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x));
    df.push("y", Series::new(y));

    // Write to netcdf file
    df.write_nc("data/data.nc")?;

    Ok(())
}
