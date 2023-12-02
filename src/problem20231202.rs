use std::path::PathBuf;
use std::cmp::max;
use crate::utils::read_lines;

use regex::Regex;

fn get_rgb(marbles: &str) -> (u64, u64, u64) {
    let mut ret: Vec<u64> = vec![0, 0, 0];
    for (idx, color) in ["red", "green", "blue"].iter().enumerate() {
        if marbles.contains(color) {
            let re = Regex::new(format!(r"\d+ {}", color).as_str()).unwrap();
            let result = re.captures(marbles).unwrap();
            ret[idx] = result[0].split(" ").into_iter().nth(0).unwrap().parse::<u64>().unwrap();
        }
    }

    return (ret[0], ret[1], ret[2]);
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input2".to_string()].iter().collect();

    let mut cnt0: u64 = 0;
    let mut cnt1: u64 = 0;
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                let game_id: u64 = num_str
                    .split(":")
                    .into_iter()
                    .nth(0)
                    .unwrap()
                    .split(" ")
                    .into_iter()
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let rgbs: Vec<(u64, u64, u64)> = num_str
                    .split(":")
                    .into_iter()
                    .nth(1)
                    .unwrap()
                    .split(";")
                    .map(get_rgb)
                    .collect();
                cnt0 += if
                    rgbs
                        .iter()
                        .map(|rgb| rgb.0 <= 12 && rgb.1 <= 13 && rgb.2 <= 14)
                        .all(|x| x)
                {
                    game_id
                } else {
                    0
                };

                let rgb_max = rgbs
                    .iter()
                    .fold((0, 0, 0), |v, rgb| (max(v.0, rgb.0), max(v.1, rgb.1), max(v.2, rgb.2)));
                cnt1 += rgb_max.0 * rgb_max.1 * rgb_max.2;
            }
        }
    }

    return (1, format!("{}", cnt0), format!("{}", cnt1));
}
