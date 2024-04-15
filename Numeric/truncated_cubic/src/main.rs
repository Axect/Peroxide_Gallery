use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = seq(-2f64, 2f64, 0.2);
    let y = x.fmap(|t| (50f64 * (t - 1f64)).tanh() - (50f64 * (t + 1f64)).tanh());

    let cs = cubic_hermite_spline(&x, &y, Quadratic)?;
    let p = |x: f64| {
        if x < -2f64 {
            -2f64
        } else if x > 0f64 {
            0f64
        } else {
            x
        }
    };

    let new_x = linspace(x[0], x[x.len() - 1], 1000);
    let y_cs = cs.eval_vec(&new_x);
    let y_cs_cond = cs.eval_vec_with_cond(&new_x, p);

    let mut plt = Plot2D::new();
    plt.set_domain(new_x)
        .insert_image(y_cs)
        .insert_image(y_cs_cond)
        .set_line_style(vec![
            (0, LineStyle::Solid),
            (1, LineStyle::Dashed),
        ])
        .set_color(vec![
            (0, "darkblue"),
            (1, "red"),
        ])
        .set_legend(vec![
            "Cubic Spline",
            "Truncated",
        ])
        .set_style(PlotStyle::Nature)
        .tight_layout()
        .set_dpi(600)
        .set_path("plot.png")
        .savefig()?;

    Ok(())
}
