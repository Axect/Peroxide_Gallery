extern crate peroxide;
use peroxide::*;

fn main() {
    let a = ml_matrix("1 3;2 3");
    let b = ml_matrix("7; 8");
    let wazd = waz_identity(&a);
    let mut x = &wazd.w.t() * &b;
    x = &wazd.z * &x;
    x.print();
}

pub struct WAZD {
    pub w: Matrix,
    pub z: Matrix,
    pub d: Matrix,
}

/// WAZ decompoition for Diagonalization
///
/// `W^T A Z = D`
#[allow(dead_code)]
fn waz_diagonal(m: &Matrix) -> WAZD {
    let n = m.row;
    let mut w = eye(n);
    let mut z = eye(n);
    let mut d = eye(n);
    let mut q = vec![0f64; n];
    let mut p = vec![0f64; n];

    for i in 0 .. n {
        let m_i = m.col(i);
        let pq = w.col(i).dot(&m_i);
        d[(i, i)] = pq;
        for j in i+1 .. n {
            q[j] = w.col(j).dot(&m_i) / pq;
            p[j] = z.col(j).dot(&m.row(i)) / pq;
        }
        for j in i+1 .. n {
            for k in 0 .. i+1 {
                w[(k, j)] -= q[j] * w[(k, i)];
                z[(k, j)] -= p[j] * z[(k, i)];
            }
        }
    }
    WAZD {
        w,
        z,
        d
    }
}

/// WAZ Decomposition for inverse
///
/// `W^T A Z = I`
fn waz_identity(m: &Matrix) -> WAZD {
    let n = m.row;
    let mut w = eye(n);
    let mut z = eye(n);
    let mut p = zeros(n, n);
    let mut q = zeros(n, n);

    for i in 0 .. n {
        let m_i = m.col(i);
        let p_ii = w.col(i).dot(&m_i);
        p[(i,i)] = p_ii;
        for j in i+1 .. n {
            q[(i, j)] = w.col(j).dot(&m_i) / p_ii;
            p[(i, j)] = z.col(j).dot(&m.row(i)) / p_ii;
            for k in 0 .. j {
                w[(k, j)] -= q[(i, j)] * w[(k, i)];
                z[(k, j)] -= p[(i, j)] * z[(k, i)];
            }
        }
        unsafe {
            let col_ptr = z.col_mut(i);
            col_ptr.into_iter().for_each(|x| *x /= p_ii);
        }
    }

    WAZD {
        w,
        z,
        d: eye(n),
    }
}
