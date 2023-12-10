use std::path::PathBuf;
use crate::utils::read_lines;

fn process_forward(r: &Vec<i64>) -> i64 {
    let mut r_copy: Vec<i64> = r.clone();

    let mut last_vals: Vec<i64> = vec![];

    last_vals.push(r_copy[r_copy.len() - 1]);

    let mut finished: bool = false;

    while !finished {
        for idx in 0..r_copy.len() - 1 {
            r_copy[idx] = r_copy[idx + 1] - r_copy[idx];
        }
        r_copy.remove(r_copy.len() - 1);

        last_vals.push(r_copy[r_copy.len() - 1]);

        finished = r_copy
            .iter()
            .map(|x| *x == 0)
            .all(|x| x);
    }

    return last_vals.into_iter().sum();
}

fn process_backward(r: &Vec<i64>) -> i64 {
    let mut r_copy: Vec<i64> = r.clone();

    let mut last_vals: Vec<i64> = vec![];

    last_vals.push(r_copy[0]);

    let mut finished: bool = false;

    while !finished {
        for idx in 0..r_copy.len() - 1 {
            r_copy[idx] = r_copy[idx + 1] - r_copy[idx];
        }
        r_copy.remove(r_copy.len() - 1);

        last_vals.push(r_copy[0]);

        finished = r_copy
            .iter()
            .map(|x| *x == 0)
            .all(|x| x);
    }
    last_vals.remove(last_vals.len() - 1);
    let mut first: i64 = 0;
    for v in last_vals.iter().rev() {
        first = v - first;
    }

    return first;
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input9".to_string()].iter().collect();

    let mut all_lines: Vec<Vec<i64>> = vec![];
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                all_lines.push(
                    num_str
                        .split(' ')
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                );
            }
        }
    }

    let cnt0: i64 = (&all_lines)
        .into_iter()
        .map(|r| process_forward(&r))
        .sum();
    let cnt1: i64 = (&all_lines)
        .into_iter()
        .map(|r| process_backward(&r))
        .sum();

    return (8, format!("{}", cnt0), format!("{}", cnt1));
}
