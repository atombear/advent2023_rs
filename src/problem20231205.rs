use std::path::PathBuf;
use crate::utils::read_lines;

fn get_seeds(line: &String) -> Vec<u64> {
    return line
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
}

fn apply_map(s: u64, map: &Vec<Vec<u64>>) -> u64 {
    let mut s: u64 = s;
    let mut dst: u64;
    let mut src: u64;
    let mut r: u64;

    for l in map {
        dst = l[0];
        src = l[1];
        r = l[2];
        if src <= s && s < src + r {
            s = dst + (s - src);
            break;
        }
    }

    return s;
}

fn apply_map_bw(s: u64, map: &Vec<Vec<u64>>) -> u64 {
    let mut s: u64 = s;
    let mut dst: u64;
    let mut src: u64;
    let mut r: u64;

    for l in map {
        dst = l[1];
        src = l[0];
        r = l[2];
        if src <= s && s < src + r {
            s = dst + (s - src);
            break;
        }
    }

    return s;
}

fn get_loc(s: u64, maps: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut s: u64 = s;
    for m in maps {
        s = apply_map(s, &m);
    }
    return s;
}

fn get_seed(l: u64, maps: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut l: u64 = l;
    for m in maps.iter().rev() {
        l = apply_map_bw(l, &m);
    }
    return l;
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input5".to_string()].iter().collect();

    let mut all_lines: Vec<String> = vec![];
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                all_lines.push(num_str);
            }
        }
    }

    let seeds0: Vec<u64> = get_seeds(&all_lines[0]);

    let mut map_lines: Vec<Vec<&String>> = vec![];
    for idx in 2..all_lines.len() {
        let line = &all_lines[idx];
        if line.contains("map") {
            map_lines.push(vec![]);
        } else if line.len() > 0 {
            let l = map_lines.len();
            map_lines[l - 1].push(line);
        }
    }

    let maps: Vec<Vec<Vec<u64>>> = map_lines
        .iter()
        .map(|v|
            v
                .iter()
                .map(|x|
                    x
                        .split(' ')
                        .filter(|x| (*x).len() > 0)
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                )
                .collect::<Vec<Vec<u64>>>()
        )
        .collect::<Vec<Vec<Vec<u64>>>>();

    let mut locs0: Vec<u64> = vec![];
    for &s in &seeds0 {
        locs0.push(get_loc(s, &maps));
    }

    let seeds1_rngs: Vec<Vec<u64>> = (0..seeds0.len() / 2)
        .into_iter()
        .map(|idx| vec![seeds0[2 * idx], seeds0[2 * idx + 1]])
        .collect::<Vec<Vec<u64>>>();

    // takes ~10s.
    let mut loc1: u64 = 99751240;
    loop {
        let seed1 = get_seed(loc1, &maps);
        if
            seeds1_rngs
                .iter()
                .map(|v| v[0] <= seed1 && seed1 < v[0] + v[1])
                .any(|x| x)
        {
            break;
        }
        loc1 += 1;
    }

    return (4, format!("{}", locs0.iter().min().unwrap()), format!("{}", loc1));
}
