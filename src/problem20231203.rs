use std::path::PathBuf;
use crate::utils::read_lines;

const DIGITS: &str = "0123456789";

fn get_numbers_locs(line: &String) -> Vec<(Vec<usize>, u64)> {
    let mut ret: Vec<(Vec<usize>, u64)> = vec![];

    let mut num: u64 = 0;
    let mut idxs: Vec<usize> = vec![];
    for (idx, c) in line.chars().enumerate() {
        if DIGITS.contains(c) {
            num *= 10;
            num += DIGITS.find(c).unwrap() as u64;
            idxs.push(idx);
        } else if idxs.len() > 0 {
            ret.push((idxs.clone(), num));
            num = 0;
            idxs = vec![];
        }
    }

    if idxs.len() > 0 {
        ret.push((idxs.clone(), num));
    }

    return ret;
}

fn get_symbol_locs(line: &String) -> Vec<usize> {
    return line
        .chars()
        .into_iter()
        .enumerate()
        .map(|(idx, c)| (idx, c != '.' && !DIGITS.contains(c)))
        .filter(|(_, b)| *b)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
}

fn get_gear_locs(line: &String) -> Vec<usize> {
    return line
        .chars()
        .into_iter()
        .enumerate()
        .map(|(idx, c)| (idx, c == '*'))
        .filter(|(_, b)| *b)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input3".to_string()].iter().collect();

    let mut all_num_locs: Vec<Vec<(Vec<usize>, u64)>> = vec![];
    let mut symbol_locs: Vec<Vec<usize>> = vec![];
    let mut gear_locs: Vec<Vec<usize>> = vec![];
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                all_num_locs.push(get_numbers_locs(&num_str));
                symbol_locs.push(get_symbol_locs(&num_str));
                gear_locs.push(get_gear_locs(&num_str));
            }
        }
    }

    let mut total0: u64 = 0;
    for row in 0..all_num_locs.len() {
        let syms = &symbol_locs[row];
        let mut idxs_nums = vec![];
        if syms.len() > 0 {
            // check above
            if row > 0 {
                idxs_nums.extend(all_num_locs[row - 1].iter());
            }
            // check same row
            idxs_nums.extend(all_num_locs[row].iter());
            // check below
            if row < all_num_locs.len() - 1 {
                idxs_nums.extend(all_num_locs[row + 1].iter());
            }
            for (idxs, num) in idxs_nums {
                for sdx in syms {
                    if idxs.contains(sdx) || idxs.contains(&(sdx + 1)) || idxs.contains(&(sdx - 1)) {
                        total0 += num;
                    }
                }
            }
        }
    }

    let mut total1: u64 = 0;
    for row in 0..all_num_locs.len() {
        let gears: &Vec<usize> = &gear_locs[row];
        let mut idxs_nums = vec![];
        if gears.len() > 0 {
            // check above
            if row > 0 {
                idxs_nums.extend(all_num_locs[row - 1].iter());
            }
            // check same row
            idxs_nums.extend(all_num_locs[row].iter());
            // check below
            if row < all_num_locs.len() - 1 {
                idxs_nums.extend(all_num_locs[row + 1].iter());
            }
            for sdx in gears {
                let mut gear_ratio: Vec<&u64> = vec![];
                for (idxs, num) in &idxs_nums {
                    if idxs.contains(sdx) || idxs.contains(&(sdx + 1)) || idxs.contains(&(sdx - 1)) {
                        gear_ratio.push(num);
                    }
                }
                if gear_ratio.len() == 2 {
                    total1 += gear_ratio[0] * gear_ratio[1];
                }
            }
        }
    }

    return (2, format!("{}", total0), format!("{}", total1));
}
