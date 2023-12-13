use std::path::PathBuf;
use crate::utils::read_lines;

const DAY: usize = 11;

type Point = (usize, usize);

fn expand_and_count(
    galaxies: &Vec<Point>,
    erows: &Vec<usize>,
    ecols: &Vec<usize>,
    exp_factor: usize
) -> u64 {
    let mut galaxies: Vec<Point> = galaxies.clone();

    for idx in 0..galaxies.len() {
        let (r, c) = galaxies[idx];
        let exp = erows
            .iter()
            .filter(|x| &&r > x)
            .count();
        galaxies[idx] = (r + exp_factor * exp, c);
    }
    for idx in 0..galaxies.len() {
        let (r, c) = galaxies[idx];
        let exp = ecols
            .iter()
            .filter(|x| &&c > x)
            .count();
        galaxies[idx] = (r, c + exp_factor * exp);
    }

    let mut total_distance: u64 = 0;
    for (n, g0) in (&galaxies).iter().enumerate() {
        for g1 in (&galaxies).iter().skip(n + 1) {
            let r0: i64 = g0.0 as i64;
            let c0: i64 = g0.1 as i64;
            let r1: i64 = g1.0 as i64;
            let c1: i64 = g1.1 as i64;

            total_distance += ((c1 - c0).abs() + (r1 - r0).abs()) as u64;
        }
    }
    return total_distance;
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

    let mut galaxies: Vec<Point> = vec![];

    for (r, row) in all_lines.iter().enumerate() {
        for (c, g) in row.chars().enumerate() {
            if g == '#' {
                galaxies.push((r, c));
            }
        }
    }

    let erows: Vec<usize> = all_lines
        .iter()
        .enumerate()
        .filter(|(_, x)| !x.contains('#'))
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();

    let ecols: Vec<usize> = (0..all_lines[0].len())
        .map(|x|
            all_lines
                .iter()
                .map(|y| y.chars().nth(x).unwrap())
                .collect::<String>()
        )
        .enumerate()
        .filter(|(_, x)| !x.contains('#'))
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();

    let total_distance0: u64 = expand_and_count(&galaxies, &erows, &ecols, 1);
    let total_distance1: u64 = expand_and_count(&galaxies, &erows, &ecols, 999999);

    return (DAY - 1, format!("{}", total_distance0), format!("{}", total_distance1));
}
