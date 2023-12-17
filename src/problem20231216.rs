use std::cmp::max;
use std::path::PathBuf;
use std::collections::HashSet;
use crate::utils::read_lines;

const DAY: usize = 16;

type Point = (usize, usize);

fn get_energized_points(
    beams: &mut HashSet<(Point, char)>,
    vsplit: &Vec<Point>,
    hsplit: &Vec<Point>,
    umirr: &Vec<Point>,
    dmirr: &Vec<Point>,
    rows: usize,
    cols: usize
) -> usize {
    let mut new_beams: HashSet<(Point, char)> = beams.clone();
    let mut temp_beams: HashSet<(Point, char)>;
    let mut energized_mirrors: HashSet<Point> = HashSet::new();

    let mut p: Point;
    let mut r: usize;
    let mut c: usize;
    let mut d: char;

    while new_beams.len() > 0 {
        temp_beams = HashSet::from_iter(new_beams.drain());
        for b in temp_beams.drain() {
            (p, d) = b;
            (r, c) = p;
            if vsplit.contains(&p) && (d == 'r' || d == 'l') {
                energized_mirrors.insert(p);
                new_beams.insert(((r - 1, c), 'u'));
                new_beams.insert(((r + 1, c), 'd'));
            } else if hsplit.contains(&p) && (d == 'u' || d == 'd') {
                energized_mirrors.insert(p);
                new_beams.insert(((r, c - 1), 'l'));
                new_beams.insert(((r, c + 1), 'r'));
            } else if umirr.contains(&p) {
                energized_mirrors.insert(p);
                match d {
                    'u' => {
                        new_beams.insert(((r, c + 1), 'r'));
                    }
                    'd' => {
                        new_beams.insert(((r, c - 1), 'l'));
                    }
                    'l' => {
                        new_beams.insert(((r + 1, c), 'd'));
                    }

                    'r' => {
                        new_beams.insert(((r - 1, c), 'u'));
                    }
                    _ => panic!("what direction is this???"),
                }
            } else if dmirr.contains(&p) {
                energized_mirrors.insert(p);
                match d {
                    'u' => {
                        new_beams.insert(((r, c - 1), 'l'));
                    }
                    'd' => {
                        new_beams.insert(((r, c + 1), 'r'));
                    }
                    'l' => {
                        new_beams.insert(((r - 1, c), 'u'));
                    }
                    'r' => {
                        new_beams.insert(((r + 1, c), 'd'));
                    }
                    _ => panic!("what direction is this???"),
                }
            } else {
                match d {
                    'u' => {
                        new_beams.insert(((r - 1, c), 'u'));
                    }
                    'd' => {
                        new_beams.insert(((r + 1, c), 'd'));
                    }
                    'l' => {
                        new_beams.insert(((r, c - 1), 'l'));
                    }
                    'r' => {
                        new_beams.insert(((r, c + 1), 'r'));
                    }
                    _ => panic!("what direction is this???"),
                }
            }
        }
        new_beams = HashSet::from_iter(
            new_beams
                .iter()
                .filter(|((r, c), d)| r < &rows && c < &cols && !beams.contains(&((*r, *c), *d)))
                .map(|((r, c), d)| ((*r, *c), *d))
        );
        beams.extend(new_beams.iter());
    }

    let energized_points: usize = HashSet::<Point>
        ::from_iter(beams.iter().map(|(p, _)| *p))
        .union(&energized_mirrors)
        .into_iter()
        .count();
    return energized_points;
}

fn get_b0bf0(idx: usize, _rows: usize, _cols: usize) -> ((Point, char), (Point, char)) {
    return (((idx, 0), 'r'), ((idx, 0), 'l'));
}

fn get_b0bf1(idx: usize, _rows: usize, _cols: usize) -> ((Point, char), (Point, char)) {
    return (((0, idx), 'd'), ((0, idx), 'u'));
}

fn get_b0bf2(idx: usize, _rows: usize, cols: usize) -> ((Point, char), (Point, char)) {
    return (((idx, cols - 1), 'l'), ((idx, cols - 1), 'r'));
}

fn get_b0bf3(idx: usize, rows: usize, _cols: usize) -> ((Point, char), (Point, char)) {
    return (((rows - 1, idx), 'u'), ((rows - 1, idx), 'd'));
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

    let mut vsplit: Vec<Point> = vec![];
    let mut hsplit: Vec<Point> = vec![];
    let mut umirr: Vec<Point> = vec![];
    let mut dmirr: Vec<Point> = vec![];

    for idx in 0..all_lines.len() {
        for (jdx, el) in all_lines[idx].chars().enumerate() {
            match el {
                '|' => vsplit.push((idx, jdx)),
                '-' => hsplit.push((idx, jdx)),
                '/' => umirr.push((idx, jdx)),
                '\\' => dmirr.push((idx, jdx)),
                _ => {}
            }
        }
    }

    let mut beams: HashSet<(Point, char)> = HashSet::new();
    beams.insert(((0, 0), 'r'));

    let rows: usize = all_lines.len();
    let cols: usize = all_lines[0].len();

    let energized_points: usize = get_energized_points(
        &mut beams,
        &vsplit,
        &hsplit,
        &umirr,
        &dmirr,
        rows,
        cols
    );

    let mut max_points: usize = 0;
    let mut all_beams: HashSet<(Point, char)> = HashSet::new();
    let mut beams: HashSet<(Point, char)> = HashSet::new();
    let mut b0: (Point, char);
    let mut bf: (Point, char);

    for (max_num, getb0bf) in [rows, cols, rows, cols]
        .iter()
        .zip([get_b0bf0, get_b0bf1, get_b0bf2, get_b0bf3]) {
        for idx in 0..*max_num {
            (b0, bf) = getb0bf(idx, rows, cols);
            if !all_beams.contains(&b0) || !all_beams.contains(&bf) {
                beams.insert(b0);
                max_points = max(
                    max_points,
                    get_energized_points(&mut beams, &vsplit, &hsplit, &umirr, &dmirr, rows, cols)
                );
                all_beams.extend(beams.drain());
            }
        }
    }

    return (DAY - 1, format!("{}", energized_points), format!("{}", max_points));
}
