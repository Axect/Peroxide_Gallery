extern crate peroxide;
use peroxide::fuga::*;

pub fn rref(m: &Matrix) -> Matrix {
    let mut lead = 0usize;
    let mut result = m.clone();
    for r in 0 .. m.row {
        if m.col <= lead {
            break;
        }
        let mut i = r;
        while result[(i, lead)] == 0f64 {
            i += 1;
            if m.row == i {
                i = r;
                lead += 1;
                if m.col == lead {
                    break;
                }
            }
        }
        unsafe {
            result.swap(i, r, Row);
        }
        let tmp = result[(r, lead)];
        if tmp != 0f64 {
            unsafe {
                result.row_mut(r).iter_mut().for_each(|t| *(*t) /= tmp);
            }
        }
        for j in 0 .. m.row {
            if j != r {
                let tmp1 = result.row(r).mul_s(result[(j, lead)]);
                let tmp2 = result.row(j).sub_v(&tmp1);
                result.subs_row(j, &tmp2);
            }
        }
        lead += 1;
    }
    result
}
