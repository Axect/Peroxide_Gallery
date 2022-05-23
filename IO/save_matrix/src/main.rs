use peroxide::fuga::*;

fn main() {
    let a = ml_matrix("1 2;3 4;5 6"); // Row shaped Matrix
    let a_shape: Vec<usize> = vec![a.row, a.col];

    // Write matrix & shape
    let mut df = DataFrame::new(vec![]);
    df.push("a", Series::new(a.as_slice().to_vec()));   // Flatten matrix
    df.push("a_shape", Series::new(a_shape));           // Put shape information

    df.write_nc("data.nc").expect("Can't write netcdf file");

    // Read matrix & shape
    let mut dg = DataFrame::read_nc("data.nc").expect("Can't read netcdf file");
    dg.as_types(vec![F64, USIZE]);                      // Ensure types (Because of USIZE)
    let b: Vec<f64> = dg["a"].to_vec();
    let r = dg["a_shape"].at_raw(0);
    let c = dg["a_shape"].at_raw(1);

    // Restore matrix
    let a_new = matrix(b, r, c, Row);

    assert_eq!(a, a_new);
}
