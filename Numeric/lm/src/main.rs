extern crate peroxide;
use peroxide::*;

fn main() {
    let noise = Normal(0., 0.5);
    let p_true: Vec<Number> = NumberVector::from_f64_vec(vec![20f64, 10f64, 1f64, 50f64]);
    let p_init = vec![5f64, 2f64, 0.2f64, 10f64];
    let domain = seq(0, 99, 1);
    let real = f(&domain, p_true.clone()).unwrap().to_f64_vec();
    let y = zip_with(|x, y| x + y, &real, &noise.sample(100));
    let data = hstack!(domain.clone(), y.clone());

    let mut opt = Optimizer::new(data, f);
    let p = opt
        .set_init_param(p_init)
        .set_max_iter(100)
        .set_method(LevenbergMarquardt)
        .optimize();
    p.print();
    opt.get_error().print();

    let mut plt = Plot2D::new();
    plt.set_domain(domain.clone())
        .insert_image(y)
        .insert_image(f(&domain, NumberVector::from_f64_vec(p)).unwrap().to_f64_vec())
        .set_legend(vec!["real", "fit"])
        .set_title("Levenberg-Marquardt Algorithm")
        .set_path("./lm_test.png")
        .set_marker(vec![Point, Line])
        .savefig()
        .expect("Can't draw a plot");
}

fn f(domain: &Vec<f64>, p: Vec<Number>) -> Option<Vec<Number>> {
    Some(
        domain.clone().into_iter()
            .map(|t| Number::from_f64(t))
            .map(|t| p[0] * (-t / p[1]).exp() + p[2] * t * (-t / p[3]).exp())
            .collect()
    )
}
