extern crate peroxide;
use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn Error>> {
    let x = seq(0, 1, 0.01);
    let x2 = x.fmap(|t| t.powi(2));
    let x3 = x.fmap(|t| t.powi(3));

    let mut plt = Plot2D::new();

    plt
        .set_domain(x.clone())
        .insert_image(x)
        .insert_image(x2)
        .insert_image(x3)
        .set_fig_size((10, 6))
        .set_dpi(300)
        .set_title("Test Plot")
        .set_xlabel("$x$")
        .set_ylabel("$y$")
        .set_legend(vec!["$y=x$", "$y=x^2$", "$y=x^3$"])
        .grid(Grid::On)
        .set_path("plot.png")
        .savefig()?;

    Ok(())
}

