use peroxide::fuga::*;

fn main() -> Result<(), RootError> {
    let init = RootState::P(1f64);
    let mut rf = RootFinder::new(init, Newton, f_ad)?;
    rf.set_tol(1e-15)
        .set_times(100);

    let mut result = vec![0f64; 100];
    let mut l = 100usize;
    result[0] = rf.get_curr().get_point().unwrap();
    for i in 1 .. 100 {
        rf.update();
        result[i] = rf.get_curr().get_point().unwrap();
        match rf.check_find() {
            RootBool::Find => {
                l = i;
                break;
            },
            RootBool::Error => {
                panic!("Error!");
            },
            _ => (),
        }
    }

    let x = seq(0, 2, 0.001);
    let y = x.fmap(f);

    let root_x = result[0..l].to_vec();
    let root_y = root_x.fmap(f);

    let mut df = DataFrame::new(vec![]);
    df.push("root_x", Series::new(root_x));
    df.push("root_y", Series::new(root_y));
    df.push("x", Series::new(x));
    df.push("y", Series::new(y));

    df.print();

    df.write_nc("data/history.nc").expect("Can't write nc file!");

    Ok(())
}

#[ad_function]
fn f(x: f64) -> f64 {
    x.exp() - (x + 2f64)
}
