use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = 1f64;
    let k = 200f64;
    let c = 0.01 * 2f64 * (k * m).sqrt();

    let x_init = 0.1f64;
    let v_init = 0f64;
    let a_init = -20f64;

    let dt = 1e-4;

    let gamma = 0.5;
    let beta = 0.25; // for average constant acceleration
    
    let mut damped_sho = NewmarkSHO::new(x_init, v_init, a_init, m, c, k, gamma, beta);

    let t_vec = seq(0f64, 10f64, dt);
    let mut x_vec = vec![0f64; t_vec.len()];
    let mut v_vec = vec![0f64; t_vec.len()];
    let mut a_vec = vec![0f64; t_vec.len()];

    for i in 0 .. t_vec.len() {
        let (x, v, a) = damped_sho.get_state();
        x_vec[i] = x;
        v_vec[i] = v;
        a_vec[i] = a;

        damped_sho.step(dt);
    }

    let mut df = DataFrame::new(vec![]);
    df.push("t", Series::new(t_vec));
    df.push("x", Series::new(x_vec));
    df.push("v", Series::new(v_vec));
    df.push("a", Series::new(a_vec));

    df.print();

    df.write_parquet("newmark_beta.parquet", CompressionOptions::Uncompressed)?;

    Ok(())
}

pub struct NewmarkSHO {
    x: f64,     // position
    v: f64,     // velocity
    a: f64,     // acceleration
    m: f64,     // mass
    c: f64,     // damping coefficient
    k: f64,     // elastic constant
    gamma: f64, // newmark gamma
    beta: f64,  // newmark beta
}

impl NewmarkSHO {
    pub fn new(x: f64, v: f64, a: f64, m: f64, c: f64, k: f64, gamma: f64, beta: f64) -> NewmarkSHO {
        NewmarkSHO {
            x,
            v,
            a,
            m,
            c,
            k,
            gamma,
            beta,
        }
    }

    pub fn get_state(&self) -> (f64, f64, f64) {
        (self.x, self.v, self.a)
    }

    pub fn step(&mut self, dt: f64) {
        let x = self.x;
        let v = self.v;
        let a = self.a;
        let m = self.m;
        let c = self.c;
        let k = self.k;
        let gamma = self.gamma;
        let beta = self.beta;

        let a_next = - (k*x + (c + k*dt)*v + ((1f64-gamma)*c*dt + 0.5 * (1f64-2f64*beta)*k*dt.powi(2))*a) / (m + c*gamma*dt + k*beta*dt.powi(2));
        let v_next = v + (1f64-gamma)*dt*a + gamma*dt*a_next;
        let x_next = x + dt*v + 0.5*dt.powi(2)*((1f64-2f64*beta)*a + 2f64*beta*a_next);

        self.x = x_next;
        self.v = v_next;
        self.a = a_next;
    }
}
