use std::path::PathBuf;
use crate::utils::read_lines;

const DAY: usize = 14;

type Point = (i64, i64);

fn transpose(p: Point, num_rows: usize) -> Point {
    let (r, c) = p;
    return (c, (num_rows as i64) - r - 1);
}

fn tilt(
    rocks: &Vec<Point>,
    blocks: &Vec<Point>,
    num_rows: usize,
    num_cols: usize
) -> (u64, Vec<Point>) {
    let mut col_rocks: Vec<Point>;
    let mut col_blocks: Vec<Point>;
    let mut last_rock: i64;
    let mut score: u64 = 0;
    let mut new_rocks: Vec<Point> = vec![];
    for col in 0..num_cols {
        col_rocks = rocks
            .iter()
            .filter(|(_, c)| *c == (col as i64))
            .map(|x| *x)
            .collect();
        col_blocks = blocks
            .iter()
            .filter(|(_, c)| *c == (col as i64))
            .map(|x| *x)
            .collect();

        col_rocks.sort();
        col_blocks.sort();

        last_rock = -1;

        for (r, _) in col_rocks {
            while col_blocks.len() > 0 && r > col_blocks[0].0 {
                last_rock = col_blocks[0].0;
                col_blocks.remove(0);
            }
            last_rock += 1;
            score += (num_rows as u64) - (last_rock as u64);
            new_rocks.push((last_rock, col as i64));
        }
    }
    return (score, new_rocks);
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
    let objects: Vec<Vec<char>> = all_lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut rocks: Vec<Point> = vec![];
    let mut blocks: Vec<Point> = vec![];

    for idx in 0..all_lines.len() {
        for jdx in 0..all_lines[0].len() {
            if objects[idx][jdx] == '#' {
                blocks.push((idx as i64, jdx as i64));
            }
            if objects[idx][jdx] == 'O' {
                rocks.push((idx as i64, jdx as i64));
            }
        }
    }

    let (score, _) = tilt(&rocks, &blocks, objects.len(), objects[0].len());

    let mut tumble_rocks: Vec<Point> = rocks.clone();
    let mut tumble_blocks: Vec<Point> = blocks.clone();
    let rows: usize = all_lines.len();
    let cols: usize = all_lines[0].len();
    let mut tumble_score: u64;

    let mut num_rows: usize;
    let mut num_cols: usize;

    let mut samples: Vec<(usize, u64)> = vec![];
    let period: usize = 11;

    for idx in 0..1000000000 {
        for jdx in 0..4 {
            num_rows = if jdx % 2 == 0 { rows } else { cols };
            num_cols = if jdx % 2 == 0 { cols } else { rows };
            // tilt
            (_, tumble_rocks) = tilt(&tumble_rocks, &tumble_blocks, num_rows, num_cols);
            // rotate
            tumble_rocks = tumble_rocks
                .iter()
                .map(|p| transpose(*p, num_rows))
                .collect();
            tumble_blocks = tumble_blocks
                .iter()
                .map(|p| transpose(*p, num_rows))
                .collect();
        }

        tumble_score = tumble_rocks
            .iter()
            .map(|(r, _)| (rows as u64) - (*r as u64))
            .sum::<u64>();

        if idx > 100 {
            samples.push((idx + 1, tumble_score));
        }
        if samples.len() == period {
            break;
        }
    }

    let nth: u64 = 1000000000;

    tumble_score = *samples
        .iter()
        .filter(|(idx, _)| (nth - (*idx as u64)) % (period as u64) == 0)
        .map(|(_, score)| score)
        .nth(0)
        .unwrap();

    return (DAY - 1, format!("{}", score), format!("{}", tumble_score));
}
