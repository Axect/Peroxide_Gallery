extern crate gaussian;
extern crate peroxide;
use gaussian::*;
use peroxide::fuga::*;

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::sync_channel(10);

    let sample = gen_sample();
    sample.write_nc("data/data.nc").expect("Can't write data.nc");
    let t: Vec<f64> = sample["y"].to_vec();

    let s = 5f64;

    for i in 1 .. 11 {
        let tx_sender = mpsc::SyncSender::clone(&tx);
        let t_sample = t.clone();
        thread::spawn(move || {
            let lam = i as f64;
            let w_reg = w_ml_reg(s, lam, &t_sample);
            let x_draw = seq(1, 100, 0.1);
            let y_draw = x_draw.fmap(|x| y(s, &w_reg, x));

            let mut df = DataFrame::new(vec![]);
            df.push("x", Series::new(x_draw));
            df.push("y", Series::new(y_draw));
            let name = format!("data/par/reg_lam_{}.nc", i);
            df.write_nc(&name).expect("Can't write reg.nc");
            tx_sender.send(i).unwrap();
        });
    }

    for (i, received) in rx.into_iter().enumerate() {
        println!("Complete {}", received);
        if i == 9 {
            break;
        }
    }
}
