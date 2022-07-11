use std::iter::repeat;

use peroxide::fuga::*;

const N: usize = 100000;

fn main() {
    let result = repeat(100).take(N).map(|n| w(n)).collect::<Vec<i64>>();
    let result2 = repeat(100).take(N).map(|n| random_walk_end(n)).collect::<Vec<i64>>();

    let mut df = DataFrame::new(vec![]);
    df.push("result", Series::new(result));
    df.push("result2", Series::new(result2));
    df.print();

    df.write_nc("result.nc").expect("Could not write to file");
}

fn w(n: usize) -> i64 {
    let bin = Binomial(n, 0.5);
    let m = bin.sample(1)[0] as usize;
    (2 * m - n) as i64
}

fn random_walk_end(n: usize) -> i64 {
    let bern = Bernoulli(0.5);
    let events = bern.sample(n);
    events.into_iter().map(|x| 2*(x as i64) - 1).sum()
}
