use peroxide::{fuga::*, hstack};

fn main() {
    let noise = Normal(0., 0.5);
    let p_true = vec![20f64, 10f64, 1f64, 50f64].into_iter()
        .map(|t| AD1(t, 1f64))
        .collect::<Vec<AD>>();
    let p_init = vec![5f64, 2f64, 0.2f64, 10f64];
    let domain = linspace(0, 100, 1000);
    let real = f(&domain, p_true.clone()).unwrap().to_f64_vec();
    let y = zip_with(|x, y| x + y, &real, &noise.sample(1000));
    let data = hstack!(domain.clone(), y.clone());

    let mut opt = Optimizer::new(data, f);
    let p = opt
        .set_init_param(p_init)
        .set_max_iter(100)
        .set_method(LevenbergMarquardt)
        .optimize();
    p.print();
    opt.get_error().print();

    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(domain.clone()));
    df.push("y", Series::new(y));
    df.push("y_hat", Series::new(f(&domain, p_true).unwrap().to_f64_vec()));
    df.print();

    df.write_parquet("data.parquet", CompressionOptions::Uncompressed).expect("Can't write parquet");
}

fn f(domain: &Vec<f64>, p: Vec<AD>) -> Option<Vec<AD>> {
    Some(
        domain.clone().into_iter()
            .map(|t| AD1(t, 0f64))
            .map(|t| p[0] * (-t / p[1]).exp() + p[2] * t * (-t / p[3]).exp())
            .collect()
    )
}
