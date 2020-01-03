extern crate peroxide;
extern crate json;
use peroxide::*;

use json::*;

// Let's plot y=x, y=x^2, y=x^3

fn main() {
    // Make dataframe
    let mut df = DataFrame::with_header(vec!["x1", "x2", "x3"]);
    df["x1"] = seq(0, 1, 0.01);
    df["x2"] = df["x1"].fmap(|x| x.powi(2));
    df["x3"] = df["x1"].fmap(|x| x.powi(3));

    // Json
}