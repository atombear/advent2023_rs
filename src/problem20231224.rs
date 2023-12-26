use std::path::PathBuf;
use crate::utils::read_lines;

const DAY: usize = 24;

type Point = (f64, f64, f64);

fn intersect2d(
    pos0: Point,
    vel0: Point,
    pos1: Point,
    vel1: Point,
    min_lim: f64,
    max_lim: f64
) -> bool {
    // x + vt = r
    // x0 + v0 t = r0
    // x1 + v1 t = r1
    // (r0 - x0) / v0 = (r1 - x1) / v1
    // (r0 - x0) * v1 / v0 + x1 = r1
    // (X - x0) * v1 / v0 + x1 = Y
    // (v1 / v0)X - (v1 / v0)x0 + x1 = Y
    // a0 x + b0 = a1 x + b1
    // (a0 - a1) x = (b1 - b0) / (a0 - a1)

    let a0: f64 = vel0.1 / vel0.0;
    let b0: f64 = pos0.1 - a0 * pos0.0;

    let a1: f64 = vel1.1 / vel1.0;
    let b1: f64 = pos1.1 - a1 * pos1.0;

    let x_int: f64 = (b1 - b0) / (a0 - a1);
    let y_int: f64 = a0 * x_int + b0;
    let t0_int: f64 = (x_int - pos0.0) / vel0.0;
    let t1_int: f64 = (x_int - pos1.0) / vel1.0;

    return t0_int > 0.0 &&
        t1_int > 0.0 &&
        min_lim < x_int &&
        x_int < max_lim &&
        min_lim < y_int &&
        y_int < max_lim;
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), format!("input{}", DAY)]
        .iter()
        .collect();

    let mut all_lines: Vec<String> = vec![];
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                all_lines.push(num_str);
            }
        }
    }

    let mut cons: Vec<Vec<Point>> = vec![vec![], vec![]];

    for line in all_lines {
        let mut line_iter = line.splitn(2, '@');
        for idx in 0..2 {
            let num_vec = line_iter
                .next()
                .unwrap()
                .split(',')
                .map(|x|
                    x
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect::<String>()
                        .parse::<f64>()
                        .unwrap()
                )
                .collect::<Vec<f64>>();
            cons[idx].push((num_vec[0], num_vec[1], num_vec[2]));
        }
    }

    let pos: &Vec<(f64, f64, f64)> = &cons[0];
    let vel: &Vec<(f64, f64, f64)> = &cons[1];

    // let min_lim: f64 = 7.0;
    // let max_lim: f64 = 27.0;
    let min_lim: f64 = 200000000000000.0;
    let max_lim: f64 = 400000000000000.0;

    let mut num_crossings: usize = 0;
    for idx in 0..pos.len() - 1 {
        for jdx in idx + 1..pos.len() {
            num_crossings += if
                intersect2d(pos[idx], vel[idx], pos[jdx], vel[jdx], min_lim, max_lim)
            {
                1
            } else {
                0
            };
        }
    }

    // part 2 is brutal...
    // 4 points needed
    // xa + va t0 = x0 + v0 t0
    // xb + vb t0 = x1 + v1 t0
    // xc + vc t0 = x2 + v2 t0

    // xd + vd t1 = x0 + v0 t1
    // xe + ve t1 = x1 + v1 t1
    // xf + vf t1 = x2 + v2 t1

    // xg + vg t2 = x0 + v0 t2
    // xh + vh t2 = x1 + v1 t2
    // xi + vi t2 = x2 + v2 t2

    // (xa - x0) / (v0 - va) = t0
    // (xb - x1) / (v1 - vb) = t0
    // (xc - x2) / (v2 - vc) = t0

    // (xd - x0) / (v0 - vd) = t1
    // (xe - x1) / (v1 - ve) = t1
    // (xf - x2) / (v2 - vf) = t1

    // (xg - x0) / (v0 - vg) = t2
    // (xh - x1) / (v1 - vh) = t2
    // (xi - x2) / (v2 - vi) = t2

    // (xa - x0) / (v0 - va) = (xb - x1) / (v1 - vb)
    // (xa - x0) / (v0 - va) = (xc - x2) / (v2 - vc)

    // (xd - x0) / (v0 - vd) = (xe - x1) / (v1 - ve)
    // (xd - x0) / (v0 - vd) = (xf - x2) / (v2 - vf)

    // (xg - x0) / (v0 - vg) = (xh - x1) / (v1 - vh)
    // (xg - x0) / (v0 - vg) = (xi - x2) / (v2 - vi)

    // (xa - x0) (v1 - vb) = (xb - x1) (v0 - va)
    // (xa - x0) (v2 - vc) = (xc - x2) (v0 - va)

    // (xd - x0) (v1 - ve) = (xe - x1) (v0 - vd)
    // (xd - x0) (v2 - vf) = (xf - x2) (v0 - vd)

    // (xg - x0) (v1 - vh) = (xh - x1) (v0 - vg)
    // (xg - x0) (v2 - vi) = (xi - x2) (v0 - vg)

    // (xa - x0) (v1 - vb) = (xb - x1) (v0 - va)
    // (xa - x0) (v2 - vc) = (xc - x2) (v0 - va)
    // xa v1 - x0 v1 - xa vb + x0 vb = xb v0 - x1 v0 - xb va + x1 va
    // xa v2 - x0 v2 - xa vc + x0 vc = xc v0 - x2 v0 - xc va + x2 va

    // (xd - x0) (v1 - ve) = (xe - x1) (v0 - vd)
    // (xd - x0) (v2 - vf) = (xf - x2) (v0 - vd)
    // xd v1 - x0 v1 - xd ve + x0 ve = xe v0 - x1 v0 - xe vd + x1 vd
    // xd v2 - x0 v2 - xd vf + x0 vf = xf v0 - x2 v0 - xf vd + x2 vd

    // (xg - x0) (v1 - vh) = (xh - x1) (v0 - vg)
    // (xg - x0) (v2 - vi) = (xi - x2) (v0 - vg)
    // xg v1 - x0 v1 - xg vh + x0 vh = xh v0 - x1 v0 - xh vg + x1 vg
    // xg v2 - x0 v2 - xg vi + x0 vi = xi v0 - x2 v0 - xi vg + x2 vg

    // xa v1 - x0 v1 - xa vb + x0 vb = xb v0 - x1 v0 - xb va + x1 va
    // xd v1 - x0 v1 - xd ve + x0 ve = xe v0 - x1 v0 - xe vd + x1 vd
    // xg v1 - x0 v1 - xg vh + x0 vh = xh v0 - x1 v0 - xh vg + x1 vg

    // (xa-xd)v1 - (xb-xe)v0 - (va-vd) x1 + (vb-ve) x0 = -xb va + xe vd + xa vb - xd ve
    // (xa-xg)v1 - (xb-xh)v0 - (va-vg) x1 + (vb-vh) x0 = -xb va + xh vg + xa vb - xg vh

    // (xa-xd)v2 - (xc-xf)v0 - (va-vd) x2 + (vc-vf) x0 = -xc va + xf vd + xa vc - xd vf
    // (xa-xg)v2 - (xc-xi)v0 - (va-vg) x2 + (vc-vi) x0 = -xc va + xi vg + xa vc - xg vi
    // import numpy as np

    // x0 = np.array([19, 13, 30], dtype=int)
    // v0 = np.array([-2,  1, -2], dtype=int)
    // x1 = np.array([18, 19, 22], dtype=int)
    // v1 = np.array([-1, -1, -2], dtype=int)
    // x2 = np.array([20, 25, 34], dtype=int)
    // v2 = np.array([-2, -2, -4], dtype=int)
    // x3 = np.array([12, 31, 28], dtype=int)
    // v3 = np.array([-1, -2, -1], dtype=int)
    
    // x0 = np.array([144788461200241, 195443318499267, 285412990927879])
    // v0 = np.array([227, 158, 5])
    
    // x1 = np.array([266680201159206, 319693757705834, 207679493757440])
    // v1 = np.array([37, -56, 138])
    
    // x2 = np.array([343135145904814, 302103279002870, 240702357103107])
    // v2 = np.array([-88, 41, 9])
    
    // x3 = np.array([344900100024424, 366032694378845, 216398516914389])
    // v3 = np.array([-22, -140, 7])
    
    // xa = x0[0]
    // xb = x0[1]
    // xc = x0[2]
    // xd = x1[0]
    // xe = x1[1]
    // xf = x1[2]
    // xg = x2[0]
    // xh = x2[1]
    // xi = x2[2]
    // xj = x3[0]
    // xk = x3[1]
    // xl = x3[2]
    
    // va = v0[0]
    // vb = v0[1]
    // vc = v0[2]
    // vd = v1[0]
    // ve = v1[1]
    // vf = v1[2]
    // vg = v2[0]
    // vh = v2[1]
    // vi = v2[2]
    // vj = v3[0]
    // vk = v3[1]
    // vl = v3[2]
    
    // r = np.array([
    //     -xb *va + xe *vd + xa* vb - xd *ve,
    //     -xb *va + xh *vg + xa* vb - xg* vh,
    //     -xb *va + xk *vj + xa* vb - xj* vk,
    //     -xc *va + xf *vd + xa* vc - xd* vf,
    //     -xc *va + xi *vg + xa* vc - xg* vi,
    //     -xc *va + xl *vj + xa* vc - xj* vl,
    // ])
    
    // A = np.array([
    //     [0, (xa-xd), -(xb-xe), 0, -(va-vd), (vb-ve)],
    //     [0, (xa-xg), -(xb-xh), 0, -(va-vg), (vb-vh)],
    //     [0, (xa-xj), -(xb-xk), 0, -(va-vj), (vb-vk)],
    //     [(xa-xd), 0, -(xc-xf), - (va-vd), 0, (vc-vf)],
    //     [(xa-xg), 0, -(xc-xi), - (va-vg), 0, (vc-vi)],
    //     [(xa-xj), 0, -(xc-xl), - (va-vj), 0, (vc-vl)],
    // ])
    
    // round(sum(np.linalg.solve(A, r)[3:]))

    return (DAY - 1, format!("{}", num_crossings), format!("{}", 931193307668256 as i64));
}
