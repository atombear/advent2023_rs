use std::path::PathBuf;
use crate::utils::read_lines;

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input6".to_string()].iter().collect();

    let mut all_lines: Vec<String> = vec![];
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                all_lines.push(num_str);
            }
        }
    }

    let times: Vec<u64> = all_lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let distances: Vec<u64> = all_lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let prod0: u64 = times
        .iter()
        .zip(distances)
        .map(|(t, d)|
            (0..*t)
                .filter(|speed| speed * (t - speed) > d)
                .map(|_| 1)
                .sum::<u64>()
        )
        .filter(|c| *c > 0)
        .product();

    let big_time: u64 = all_lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let big_distance: u64 = all_lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let a: f64 = 1.0;
    let b: f64 = -1.0 * (big_time as f64);
    let c: f64 = big_distance as f64;

    let r0: f64 = (-b + (b * b - 4.0 * a * c).sqrt()) / 2.0 / a;
    let r1: f64 = (-b - (b * b - 4.0 * a * c).sqrt()) / 2.0 / a;

    return (5, format!("{}", prod0), format!("{}", (1.0 + r0 - r1) as u64));
}
