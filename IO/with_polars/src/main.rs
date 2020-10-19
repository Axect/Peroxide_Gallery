#[macro_use]
extern crate peroxide;
extern crate polars;

use peroxide::fuga::*;
use polars::prelude::*;
use std::fs::File;
use std::io::{BufWriter, BufReader};

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let x = c!(1,2,3);
    let y = x.fmap(|t| t.powi(2));
    let z = ["a", "b", "c"];

    let s0 = Series::new("x", x.as_slice());
    let s1 = Series::new("y", y.as_slice());
    let s2 = Series::new("z", &z);

    let mut df = DataFrame::new(vec![s0, s1, s2])?;

    println!("{}", df);

    // Write IPC
    let file = File::create("data/test.dat")?;
    let mut buf = BufWriter::new(file);
    IPCWriter::new(&mut buf).finish(&mut df)?;

    // Read IPC
    let file_read = File::open("data/test.dat")?;
    let buf_read = BufReader::new(file_read);
    let df_read = IPCReader::new(buf_read).finish()?;
    
    println!("{}", df_read);

    Ok(())
}
