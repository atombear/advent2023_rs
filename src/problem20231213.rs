use std::path::PathBuf;
use crate::utils::read_lines;

const DAY: usize = 13;

fn transpose(pat: &Vec<String>) -> Vec<String> {
    let cols: usize = pat[0].len();
    return (0..cols)
        .map(|idx|
            pat
                .iter()
                .map(|l| l.chars().nth(idx).unwrap())
                .collect::<String>()
        )
        .collect::<Vec<String>>();
}

fn find_reflect_plane(pat: &Vec<String>) -> Option<usize> {
    let mut jdx: usize;
    let mut kdx: usize;
    for idx in 0..pat.len() - 1 {
        if pat[idx] == pat[idx + 1] {
            jdx = idx;
            kdx = idx + 1;
            loop {
                if pat[jdx] == pat[kdx] && (jdx == 0 || kdx == pat.len() - 1) {
                    return Some(idx);
                } else if pat[jdx] != pat[kdx] {
                    break;
                } else {
                    jdx -= 1;
                    kdx += 1;
                }
            }
        }
    }
    return None;
}

fn find_reflect_plane_smudge(pat: &Vec<String>) -> Option<usize> {
    let mut jdx: usize;
    let mut kdx: usize;
    let mut smudges: usize;
    for idx in 0..pat.len() - 1 {
        jdx = idx;
        kdx = idx + 1;
        smudges = 0;
        loop {
            smudges += pat[jdx]
                .chars()
                .zip(pat[kdx].chars())
                .map(|(c0, c1)| c0 != c1)
                .filter(|x| *x)
                .count();
            if smudges > 1 {
                break;
            } else if jdx == 0 || kdx == pat.len() - 1 {
                if smudges == 1 {
                    return Some(idx);
                } else {
                    break;
                }
            } else {
                jdx -= 1;
                kdx += 1;
            }
        }
    }
    return None;
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

    let mut patterns: Vec<Vec<String>> = vec![vec![]];
    for line in all_lines {
        if line.len() == 0 {
            patterns.push(vec![]);
        } else {
            let l: usize = patterns.len();
            patterns[l - 1].push(line);
        }
    }

    let mut left_count0: u64 = 0;
    let mut top_count0: u64 = 0;
    for pat in &patterns {
        left_count0 += match find_reflect_plane(&transpose(&pat)) {
            Some(val) => { 1 + (val as u64) }
            None => { 0 }
        };
        top_count0 += match find_reflect_plane(&pat) {
            Some(val) => { 1 + (val as u64) }
            None => { 0 }
        };
    }

    let mut left_count1: u64 = 0;
    let mut top_count1: u64 = 0;
    for pat in &patterns {
        left_count1 += match find_reflect_plane_smudge(&transpose(&pat)) {
            Some(val) => { 1 + (val as u64) }
            None => { 0 }
        };
        top_count1 += match find_reflect_plane_smudge(&pat) {
            Some(val) => { 1 + (val as u64) }
            None => { 0 }
        };
    }

    return (
        DAY - 1,
        format!("{}", 100 * top_count0 + left_count0),
        format!("{}", 100 * top_count1 + left_count1),
    );
}
