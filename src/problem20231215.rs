use std::path::PathBuf;
use crate::utils::read_lines;
use ascii_converter::string_to_decimals;

fn hash(word: &str) -> u64 {
    let mut ret: u64 = 0;

    for n in string_to_decimals(word).unwrap() {
        ret += n as u64;
        ret *= 17;
        ret = ret % 256;
    }
    return ret;
}

const DAY: usize = 15;

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

    let data: &String = &all_lines[0];

    let hash_result: u64 = data
        .split(',')
        .map(|x| hash(x))
        .sum::<u64>();

    let mut lenses: Vec<Vec<(&str, u64)>> = vec![];
    for _ in 0..256 {
        lenses.push(vec![]);
    }
    let mut lens: &str;
    let mut box_num: usize;
    let mut f: u64;
    let mut lens_box: &mut Vec<(&str, u64)>;
    for lens_f_str in data.split(',') {
        if lens_f_str.contains('=') {
            lens = lens_f_str.split('=').nth(0).unwrap();
            box_num = hash(lens) as usize;
            f = lens_f_str.split('=').nth(1).unwrap().parse::<u64>().unwrap();
            lens_box = lenses[box_num].as_mut();

            if
                let Some(idx) = lens_box
                    .iter()
                    .enumerate()
                    .filter(|(_, (l, _))| *l == lens)
                    .map(|(idx, _)| idx)
                    .nth(0)
            {
                lens_box[idx] = (lens, f);
            } else {
                lens_box.push((lens, f));
            }
        } else {
            lens = lens_f_str.split('-').nth(0).unwrap();
            box_num = hash(lens) as usize;
            lens_box = lenses[box_num as usize].as_mut();

            if
                let Some(idx) = lens_box
                    .iter()
                    .enumerate()
                    .filter(|(_, (l, _))| *l == lens)
                    .map(|(idx, _)| idx)
                    .nth(0)
            {
                lens_box.remove(idx);
            }
        }
    }

    let focusing_power: u64 = lenses
        .iter()
        .enumerate()
        .flat_map(|(idx, lens_box)|
            lens_box
                .iter()
                .enumerate()
                .map(move |(jdx, (_, f))| (1 + (idx as u64)) * (1 + (jdx as u64)) * f)
        )
        .sum::<u64>();

    return (DAY - 1, format!("{}", hash_result), format!("{}", focusing_power));
}
