use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = seq(-2f64, 2f64, 0.2);
    let y = x.fmap(|t| (50f64 * (t - 1f64)).tanh() - (50f64 * (t + 1f64)).tanh());

    let cs = cubic_spline(&x, &y)?;
    let cs_akima = cubic_hermite_spline(&x, &y, Akima)?;
    let cs_quad = cubic_hermite_spline(&x, &y, Quadratic)?;

    let new_x = linspace(x[0], x[x.len() - 1], 1000);
    let y_cs = cs.eval_vec(&new_x);
    let y_akima = cs_akima.eval_vec(&new_x);
    let y_quad = cs_quad.eval_vec(&new_x);

    let mut plt = Plot2D::new();
    plt.insert_pair((x, y))
        .insert_pair((new_x.clone(), y_cs))
        .insert_pair((new_x.clone(), y_akima))
        .insert_pair((new_x, y_quad))
        .set_plot_type(vec![(0, PlotType::Scatter)])
        .set_marker(vec![(0, Markers::Point)])
        .set_line_style(vec![
            (1, LineStyle::Solid),
            (2, LineStyle::Dashed),
            (3, LineStyle::Dotted),
        ])
        .set_color(vec![
            (0, "gray"),
            (1, "red"),
            (2, "darkblue"),
            (3, "darkgreen"),
        ])
        .set_legend(vec![
            "Data",
            "Cubic Spline",
            "Cubic Hermite Spline (Akima)",
            "Cubic Hermite Spline (Quadratic)",
        ])
        .set_style(PlotStyle::Nature)
        .tight_layout()
        .set_dpi(600)
        .set_path("plot.png")
        .savefig()?;

    Ok(())
}
