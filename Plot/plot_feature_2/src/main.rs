extern crate peroxide;
use peroxide::fuga::*;

use std::collections::BTreeMap;
// ## alternative (unordered creation)
//use std::collections::HashMap;

trait PlotStyleExt {
    fn from_str(s: &str) -> PlotStyle;
}

impl PlotStyleExt for PlotStyle {
    fn from_str(s: &str) -> PlotStyle {
        match s {
            //"Default" => PlotStyle::Default,
            "IEEE" => PlotStyle::IEEE,
            "Nature" => PlotStyle::Nature,
            "Science" => PlotStyle::Science,
            _ => PlotStyle::Default
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let x = seq(0, 1, 0.01);
    let x2 = x.fmap(|t| t.powi(2));
    let x3 = x.fmap(|t| t.powi(3));

    let mut styles: BTreeMap<&str, &str> = BTreeMap::new();
    // ## alternatively create an unordered mapping
    //let mut styles: HashMap<&str, &str> = HashMap::new();
    //
    // ## supported scienceplot styles (hard coded enum ...)
    // ## no direct setting of e.g. "retro" possible
    styles.insert("ieee", "IEEE");
    // ## unordered HashTable works sometimes for all styles
    // ## orderer BTreeMap fails with some latex error for Nature style
    //styles.insert("nature", "Nature");
    styles.insert("science", "Science");
    // needs to be (alphabetically) first entry, 
    // otherwise Default has properties of previous style!
    styles.insert("0_default", "Default");

    for (key, style_name) in styles.iter() {

        let mut plt = Plot2D::new();

        let mut filename: String = "plot".to_owned();
        if (*key) != "0_default" {
            filename = format!("{}_{}.png", filename, key);
        } else {
            filename = format!("{}.png", filename);
        }
        print!("{} ...", filename);

        plt
            .set_domain(x.clone())
            .insert_image(x.clone())
            .insert_image(x2.clone())
            .insert_image(x3.clone())
            //.set_style(PlotStyle::Default)
            .set_style(PlotStyle::from_str(style_name))
            .set_dpi(300)
            .set_xlabel("$x$")
            .set_ylabel("$y$")
            .set_legend(vec!["$y=x$", "$y=x^2$", "$y=x^3$"])
            .grid(Grid::On)
            .tight_layout()
            .set_path(filename.as_str())
            .savefig()?;

        println!(" saved!");
    }

    Ok(())
}
