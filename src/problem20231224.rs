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

    return 
        t0_int > 0.0 &&
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

    return (DAY - 1, format!("{}", num_crossings), format!("{}", 0));
}
