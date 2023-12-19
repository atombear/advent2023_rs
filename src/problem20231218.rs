use std::path::PathBuf;
use std::collections::HashMap;
use crate::utils::read_lines;

const DAY: usize = 18;

type Point = (i64, i64);

fn count_area(row: i64, row_cols: &Vec<i64>, vpnts: &Vec<(Point, i64)>) -> u64 {
    let mut ret: u64 = 0;

    for idx in 0..row_cols.len() - 1 {
        if row_cols[idx + 1] - row_cols[idx] > 1 {
            if vpnts.iter().any(|((r, c), s)| *c == row_cols[idx] && *r >= row && row >= r - s) {
                ret += (row_cols[idx + 1] - row_cols[idx] - 1) as u64;
            }
        }
    }

    return ret;
}

fn l2d(l: char) -> char {
    match l {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("OH NO"),
    }
}

fn to_decimal(c: char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => panic!("not hex!"),
    }
}

fn real_dist(s: &str) -> usize {
    let mut ret: usize = 0;
    let cs: Vec<char> = s.chars().collect::<Vec<char>>();

    for idx in 1..6 {
        ret += ((16_i32).pow(5 - idx) * to_decimal(cs[idx as usize])) as usize;
    }

    return ret;
}

fn advance_in_d(d: char) -> fn(Point) -> Point {
    match d {
        'U' => |(r, c)| (r - 1, c),
        'L' => |(r, c)| (r, c - 1),
        'D' => |(r, c)| (r + 1, c),
        'R' => |(r, c)| (r, c + 1),
        _ => panic!("UNKNOWN DIRECTION!"),
    }
}

fn get_small_lagoon_vol(all_row_cols: &HashMap<i64, Vec<i64>>, vpnts: &Vec<(Point, i64)>) -> u64 {
    let row_min: i64 = all_row_cols
        .iter()
        .map(|(k, _)| *k)
        .min()
        .unwrap();
    let row_max: i64 = all_row_cols
        .iter()
        .map(|(k, _)| *k)
        .max()
        .unwrap();

    let mut total_volume: u64 = 0;
    for r in row_min..row_max + 1 {
        total_volume += count_area(r, &all_row_cols.get(&r).unwrap(), vpnts);
    }

    let lagoon_volume: u64 =
        total_volume +
        all_row_cols
            .iter()
            .map(|(_, l)| l.len() as u64)
            .sum::<u64>() -
        1;

    return lagoon_volume;
}

fn find_interior(dirs: &Vec<char>, steps: &Vec<usize>) -> u64 {
    let mut all_row_calls: HashMap<i64, Vec<i64>> = HashMap::new();
    all_row_calls.insert(0, vec![0]);
    let mut vpnts: Vec<(Point, i64)> = vec![];
    {
        let mut p: Point = (0, 0);
        for (d, s) in dirs.iter().zip(steps) {
            if *d == 'U' {
                vpnts.push((p, *s as i64));
            }
            for _ in 0..*s {
                p = advance_in_d(*d)(p);
                let (r, c) = p;
                if !all_row_calls.contains_key(&r) {
                    all_row_calls.insert(r, vec![]);
                }
                all_row_calls.get_mut(&r).unwrap().push(c);
            }
        }
    }

    for (_, l) in all_row_calls.iter_mut() {
        l.sort();
    }

    return get_small_lagoon_vol(&all_row_calls, &vpnts);
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

    let dirs: Vec<char> = all_lines
        .iter()
        .map(|x| x.chars().nth(0).unwrap())
        .collect::<Vec<char>>();
    let steps: Vec<usize> = all_lines
        .iter()
        .map(|x| x.split(' ').nth(1).unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let colors: Vec<&str> = all_lines
        .iter()
        .map(|x| x.split(' ').nth(2).unwrap().strip_prefix('(').unwrap().strip_suffix(')').unwrap())
        .collect::<Vec<&str>>();

    let real_dirs: Vec<char> = colors
        .iter()
        .map(|s| l2d(s.chars().nth(6).unwrap()))
        .collect::<Vec<char>>();
    let real_steps: Vec<usize> = colors
        .iter()
        .map(|s| real_dist(s))
        .collect::<Vec<usize>>();

    let vol_small: u64 = find_interior(&dirs, &steps);
    let vol_large: u64 = find_interior(&real_dirs, &real_steps);
    return (DAY - 1, format!("{}", vol_small), format!("{}", vol_large));
}
