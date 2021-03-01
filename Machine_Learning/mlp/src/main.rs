use peroxide::fuga::*;

// x : n x L
// xb: n x (L+1)
// v : (L+1) x M
// a : n x M
// ab: n x (M+1)
// w : (M+1) x n
// wb: M x N
// y : n x N
// t : n x N
// dh: n x M
// do: n x N

fn main() {
    let v = weights_init(3, 2);
    let w = weights_init(3, 1);

    let x = ml_matrix("0 0; 0 1; 1 0; 1 1");
    let t = ml_matrix("0;1;1;0");

    println!("Input: ");
    x.print();
    println!();

    println!("True Output: ");
    t.print();
    println!();

    let y = train(v, w, x, t, 0.25, 20000);
    println!("Predict: ");
    y.print();
}

fn weights_init(m: usize, n: usize) -> Matrix {
    rand(m, n).change_shape() * 2f64 - 1f64
}

#[ad_function]
fn sigmoid(x: f64) -> f64 {
    1f64 / (1f64 + (-x).exp())
}

#[ad_function]
fn tanh(x: f64) -> f64 {
    x.tanh()
}

fn forward(weights: &Matrix, input_bias: &Matrix) -> Matrix {
    let s = input_bias * weights;
    s.fmap(|x| sigmoid(x))
}

fn add_bias(input: &Matrix, bias: f64) -> Matrix {
    let b = matrix(vec![bias; input.row], input.row, 1, Col);
    cbind(b, input.clone())
}

fn hide_bias(weight: &Matrix) -> Matrix {
    weight.skip_row(1)
}

fn train(
    weights1: Matrix,
    weights2: Matrix,
    input: Matrix,
    answer: Matrix,
    eta: f64,
    times: usize,
) -> Matrix {
    let x = input;
    let mut v = weights1;
    let mut w = weights2;
    let t = answer;
    let xb = add_bias(&x, -1f64);

    // Vectorize gradient of activation function
    //let dsigmoid = |m: &Matrix| m.fmap(|x| sigmoid_grad(x));
    let dtanh = |m: &Matrix| m.fmap(|x| tanh_grad(x));

    let d_act = dtanh;

    for _i in 0..times {
        let a = forward(&v, &xb);
        let ab = add_bias(&a, -1f64);
        let y = forward(&w, &ab);
        let wb = hide_bias(&w);
        let delta_o = (&y - &t).hadamard(&d_act(&y));
        let delta_h = (&delta_o * &wb.t()).hadamard(&d_act(&a));
        //let delta_o = (&y - &t).hadamard(&y.hadamard(&(-&y + 1f64)));
        //let delta_h = (&delta_o * &wb.t()).hadamard(&a.hadamard(&(-&a + 1f64)));

        w = w.clone() - eta * (ab.t() * delta_o);
        v = v.clone() - eta * (xb.t() * delta_h);
    }

    let a = forward(&v, &xb);
    let ab = add_bias(&a, -1f64);
    let y = forward(&w, &ab);

    y
}

// Older version (0.30.0)

//// adfn is for activation
//fn forward<F>(weights: &Matrix, input_bias: &Matrix, adfn: &ADFn<F>) -> Matrix 
//where F: Fn(AD) -> AD 
//{
//    let s = input_bias * weights;
//    s.fmap(|x| adfn.call_stable(x))
//}
//
//fn add_bias(input: &Matrix, bias: f64) -> Matrix {
//    let b = matrix(vec![bias; input.row], input.row, 1, Col);
//    cbind(b, input.clone())
//}
//
//fn hide_bias(weight: &Matrix) -> Matrix {
//    weight.skip_row(1)
//}
//
//fn train(
//    weights1: Matrix,
//    weights2: Matrix,
//    input: Matrix,
//    answer: Matrix,
//    eta: f64,
//    times: usize,
//) -> Matrix {
//    let x = input;
//    let mut v = weights1;
//    let mut w = weights2;
//    let t = answer;
//    let xb = add_bias(&x, -0.1f64);
//
//    // Choose activation function - sigmoid or tanh
//    //let act = ADFn::new(sigmoid);
//    let act = ADFn::new(tanh);
//    //let act = ADFn::new(relu);
//    // gradient of sigmoid
//    let d_act = act.grad();
//    // vectorize function
//    let dv_act = |m: &Matrix| m.fmap(|x| d_act.call_stable(x));
//
//    for _i in 0..times {
//        let a = forward(&v, &xb, &act);
//        let ab = add_bias(&a, -0.1f64);
//        let y = forward(&w, &ab, &act);
//        //let err = (&y - &t).t() * (&y - &t);
//        //err[(0,0)].print();
//        let wb = hide_bias(&w);
//        let delta_o = (&y - &t).hadamard(&dv_act(&y));
//        let delta_h = (&delta_o * &wb.t()).hadamard(&dv_act(&a));
//
//        w = w.clone() - eta * (ab.t() * delta_o);
//        v = v.clone() - eta * (xb.t() * delta_h);
//    }
//
//    let a = forward(&v, &xb, &act);
//    let ab = add_bias(&a, -0.1f64);
//    let y = forward(&w, &ab, &act);
//
//    y
//}
