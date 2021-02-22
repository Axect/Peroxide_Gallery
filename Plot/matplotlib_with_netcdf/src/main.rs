extern crate peroxide;
use peroxide::fuga::*;
use std::error::Error;

// Let's draw a plot y = x, y = x^2, y=x^3

fn main() -> Result<(), Box<dyn Error>> {
    // First make a domain vector (0 to 1 with stepsize 0.01)
    let x = seq(0, 1, 0.01);
    
    // Second make x^2
    let x2 = x.fmap(|t| t.powi(2));

    // Third make x^3
    let x3 = x.fmap(|t| t.powi(3));

    // Create dataframe
    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x));
    df.push("x2", Series::new(x2));
    df.push("x3", Series::new(x3));

    // Write dataframe to netcdf format
    df.write_nc("data/plot.nc")?;

    Ok(())
}
