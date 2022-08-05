use peroxide::fuga::*;

fn main() {
    let k = 200f64;
    let f = |x: f64| sho(x, k);

    let mut verlet_integrator = Verlet::new(0.1, 0f64, f);
    let dt = 1e-4;
    
    let t = seq(0, 10, 1e-4);
    let mut x = vec![0f64; t.len()];
    let mut v = vec![0f64; t.len()];

    for i in 0 .. t.len() {
        x[i] = verlet_integrator.get_x();
        v[i] = verlet_integrator.get_v();

        verlet_integrator.step(dt);
    }

    let mut df = DataFrame::new(vec![]);
    df.push("t", Series::new(t));
    df.push("x", Series::new(x));
    df.push("v", Series::new(v));

    df.print();

    df.write_nc("data/verlet.nc").expect("failed to write data");
}

fn sho(x: f64, k: f64) -> f64 {
    -k * x
}

struct Verlet<F: Fn(f64) -> f64 + Copy> {
    x: f64,
    v: f64,
    a: F
}

impl<F: Fn(f64) -> f64 + Copy> Verlet<F> {
    fn new(x: f64, v: f64, a: F) -> Verlet<F> {
        Verlet { x, v, a }
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_v(&self) -> f64 {
        self.v
    }

    fn step(&mut self, dt: f64) {
        let x = self.x;
        let v = self.v;

        let v_half = v + dt * (self.a)(x) / 2.0;
        let x_new = x + dt * v_half;

        let a_new = (self.a)(x_new);
        let v_new = v_half + dt * a_new / 2.0;

        self.x = x_new;
        self.v = v_new;
    }
}

