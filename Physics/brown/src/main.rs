use peroxide::fuga::*;

fn main() {
    let bin = Bernoulli(0.5);
    let event = bin.sample(10000);
    
    let trial = seq(0, event.len() as i32, 1);
    let mut position = 0i64;
    let mut path = vec![0i64; trial.len()];
    for i in 1 .. trial.len() {
        if event[i-1] == 1f64 {
            position += 1;
        } else if event[i-1] == 0f64 {
            position -= 1;
        } else {
            panic!("event {} is not 0 or 1", i);
        }
        path[i] = position;
    }

    let mut df = DataFrame::new(vec![]);
    df.push("trial", Series::new(trial));
    df.push("path", Series::new(path));
    df.print();

    df.write_nc("path.nc").expect("Can't write path");
}
