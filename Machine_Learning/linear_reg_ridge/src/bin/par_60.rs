extern crate gaussian;
extern crate peroxide;
use gaussian::*;
use peroxide::*;

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::sync_channel(60);

    let sample = gen_sample();
    sample.write_nc("data/data.nc").expect("Can't write data.nc");
    let x = &sample["x"];
    let t = &sample["y"];

    let s = 5f64;

    for i in 1 .. 61 {
        let tx_sender = mpsc::SyncSender::clone(&tx);
        let t_sample = t.clone();
        thread::spawn(move || {
            let lam = i as f64;
            let w_reg = w_ml_reg(s, lam, &t_sample);
            let x_draw = seq(1, 100, 0.1);
            let y_draw = x_draw.fmap(|x| y(s, &w_reg, x));

            let mut df = DataFrame::with_header(vec!["x", "y"]);
            df["x"] = x_draw;
            df["y"] = y_draw;
            let name = format!("data/par/reg_lam_{}.nc", i);
            df.write_nc(&name).expect("Can't write reg.nc");
            tx_sender.send(i).unwrap();
        });
    }

    for (i, received) in rx.into_iter().enumerate() {
        println!("Complete {}", received);
        if i == 59 {
            break;
        }
    }

    //let lam = 1f64;
    ////let w = w_mle(s, t); // Just regression
    //let w_reg = w_ml_reg(s, lam, t);

    //let x_draw = seq(1, 100, 0.1);
    //let y_draw = x_draw.fmap(|x| y(s, &w_reg, x));

    //let mut df = DataFrame::with_header(vec!["x", "y"]);
    //df["x"] = x_draw;
    //df["y"] = y_draw;

    ////df.write_nc("reg.nc").expect("Can't write reg.nc");
    //df.write_nc("reg_lam_1.nc").expect("Can't write reg_lam_1.nc");
}
