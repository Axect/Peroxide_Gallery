use peroxide::fuga::*;

const N: usize = 1000;

#[allow(non_snake_case)]
fn main() {
    let n1_x = Normal(2f64, 1f64);
    let n1_y = Normal(1f64, 1.5f64);
    let n2_x = Normal(-1f64, 1f64);
    let n2_y = Normal(-2f64, 1.5f64);

    let x1 = n1_x.sample(N);
    let y1 = n1_y.sample(N);
    let c1 = vec![1f64; N];
    let x2 = n2_x.sample(N);
    let y2 = n2_y.sample(N);
    let c2 = vec![-1f64; N];

    let X1 = matrix(concat(&x1, &y1), N, 2, Row);
    let X2 = matrix(concat(&x2, &y2), N, 2, Row);

    let X = rbind(X1, X2);
    let y = concat(&c1, &c2);

    let mut svm = SVM::new(1e-3, 1e-2, 1000);
    let base_pred = svm.baseline(&X);
    let base_cm = ConfusionMatrix::new(&y, &base_pred);
    base_cm.summary();

    svm.fit(&X, &y);
    let y_hat = svm.predict(&X);
    let cm = ConfusionMatrix::new(&y, &y_hat);
    cm.summary();

    let AB = platt_scaling(&y, &y_hat);

    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(X.col(0)));
    df.push("y", Series::new(X.col(1)));
    df.push("g", Series::new(y));
    df.push("g_hat", Series::new(y_hat));
    df.push("w", Series::new(svm.w.clone()));
    df.push("b", Series::new(vec![svm.b]));
    df.push("A", Series::new(vec![AB.0]));
    df.push("B", Series::new(vec![AB.1]));

    df.print();

    df.write_parquet("svm.parquet", CompressionOptions::Uncompressed).unwrap();
}

struct SVM {
    lr: f64,
    lambda: f64,
    n_iters: usize,
    w: Vec<f64>,
    b: f64,
    cls_map: Vec<f64>,
}

impl SVM {
    fn new(lr: f64, lambda: f64, n_iters: usize) -> Self {
        Self {
            lr,
            lambda,
            n_iters,
            w: vec![0f64],
            b: 0f64,
            cls_map: vec![0f64],
        }
    }

    fn init_weight(&mut self, x: &Matrix) {
        self.w = vec![0f64; x.col];
    }

    fn get_cls_map(&mut self, y: &Vec<f64>) {
        self.cls_map = y.iter().map(|&x| if x == 1f64 { 1f64 } else { -1f64 }).collect();
    }

    fn satisfy_constraint(&self, x: &Vec<f64>, idx: usize) -> bool {
        let linear_model = self.w.dot(x) + self.b;
        let y = self.cls_map[idx];
        linear_model * y >= 1f64
    }

    fn get_gradients(&self, constrain: bool, x: &Vec<f64>, idx: usize) -> (Vec<f64>, f64) {
        if constrain {
            (self.w.mul_s(self.lambda), 0f64)
        } else {
            let y = self.cls_map[idx];
            let dw = self.w.iter().zip(x.iter()).map(|(&w, &x)| self.lambda * w - y * x).collect();
            let db = -y;
            (dw, db)
        }
    }

    fn update_weight_bias(&mut self, dw: Vec<f64>, db: f64) {
        self.w = self.w.iter().zip(dw.iter()).map(|(&w, &dw)| w - self.lr * dw).collect();
        self.b = self.b - self.lr * db;
    }

    #[allow(non_snake_case)]
    fn fit(&mut self, X: &Matrix, y: &Vec<f64>) {
        self.init_weight(X);
        self.get_cls_map(y);

        for _ in 0..self.n_iters {
            for i in 0 .. X.row {
                let x = X.row(i);
                let constrain = self.satisfy_constraint(&x, i);
                let (dw, db) = self.get_gradients(constrain, &x, i);
                self.update_weight_bias(dw, db);
            }
        }
    }

    #[allow(non_snake_case)]
    fn compute_decision_values(&self, X: &Matrix) -> Vec<f64> {
        X.apply(&self.w).add_s(self.b)
    } 

    #[allow(non_snake_case)]
    fn predict(&self, X: &Matrix) -> Vec<f64> {
        let estimate = self.compute_decision_values(X);
        let prediction = estimate.fmap(|t| if t > 0f64 { 1f64 } else { -1f64 });
        prediction
    }

    #[allow(non_snake_case)]
    fn baseline(&mut self, X: &Matrix) -> Vec<f64> {
        self.w = vec![0f64; X.col];
        self.predict(X)
    }
}

#[allow(non_snake_case)]
struct ConfusionMatrix {
    TP: usize,
    TN: usize,
    FP: usize,
    FN: usize,
}

impl ConfusionMatrix {
    #[allow(non_snake_case)]
    fn new(y: &Vec<f64>, y_hat: &Vec<f64>) -> Self {
        let mut TP = 0;
        let mut TN = 0;
        let mut FP = 0;
        let mut FN = 0;

        for (&y, &y_hat) in y.iter().zip(y_hat.iter()) {
            if y == 1f64 && y_hat == 1f64 {
                TP += 1;
            } else if y == -1f64 && y_hat == -1f64 {
                TN += 1;
            } else if y == -1f64 && y_hat == 1f64 {
                FP += 1;
            } else if y == 1f64 && y_hat == -1f64 {
                FN += 1;
            }
        }

        Self {
            TP,
            TN,
            FP,
            FN,
        }
    }

    fn accuracy(&self) -> f64 {
        (self.TP + self.TN) as f64 / (self.TP + self.TN + self.FP + self.FN) as f64
    }

    fn precision(&self) -> f64 {
        self.TP as f64 / (self.TP + self.FP) as f64
    }

    fn recall(&self) -> f64 {
        self.TP as f64 / (self.TP + self.FN) as f64
    }

    fn f1_score(&self) -> f64 {
        let p = self.precision();
        let r = self.recall();
        2f64 * p * r / (p + r)
    }

    fn specificity(&self) -> f64 {
        self.TN as f64 / (self.TN + self.FP) as f64
    }

    fn npv(&self) -> f64 {
        self.TN as f64 / (self.TN + self.FN) as f64
    }

    fn to_matrix(&self) -> Matrix {
        let mut m = zeros(2, 2);
        m[(0, 0)] = self.TP as f64;
        m[(0, 1)] = self.FP as f64;
        m[(1, 0)] = self.FN as f64;
        m[(1, 1)] = self.TN as f64;
        m
    }

    fn summary(&self) {
        println!("Accuracy:\t{:.2}", self.accuracy());
        println!("Precision:\t{:.2}", self.precision());
        println!("Recall:\t\t{:.2}", self.recall());
        println!("F1 Score:\t{:.2}", self.f1_score());
        println!("Specificity:\t{:.2}", self.specificity());
        println!("NPV:\t\t{:.2}", self.npv());
    }
}

#[allow(non_snake_case)]
fn platt_scaling(y: &Vec<f64>, y_hat: &Vec<f64>) -> (f64, f64) {
    let N_p = y.iter().filter(|&&x| x == 1f64).count();
    let N_n = y.iter().filter(|&&x| x == -1f64).count();
    let t_p = (1f64 + N_p as f64) / (2f64 * N_p as f64);
    let t_n = 1f64 / (2f64 + 2f64 * N_n as f64);

    let x = y_hat.clone();
    let y = y.clone().fmap(|t| if t == 1f64 { t_p } else { t_n });

    let data = matrix(concat(&x, &y), x.len(), 2, Col);

    let mut opt = Optimizer::new(data, logistic_transform);
    let AB = opt.set_init_param(vec![1f64, 1f64])
        .set_max_iter(100)
        .set_method(LevenbergMarquardt)
        .set_lambda_init(1e-4)
        .set_lambda_max(1e+3)
        .optimize();
    (AB[0], AB[1])
}

#[allow(non_snake_case)]
fn logistic_transform(x: &Vec<f64>, AB: Vec<AD>) -> Option<Vec<AD>> {
    Some(
        x.clone().into_iter()
            .map(|t| AD1(t, 0f64))
            .map(|t| 1f64 / (AB[0] * t + AB[1]).exp())
            .collect()
    )
}
