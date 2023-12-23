use std::path::PathBuf;
use std::collections::HashSet;
use crate::utils::read_lines;

const DAY: usize = 21;

type PointGrid = (usize, usize);
type Point = (usize, usize, i64, i64);

fn adj_points(p: &Point, rows: &usize, cols: &usize) -> Vec<Point> {
    let (r, c, rr, cc) = *p;
    let rs: (usize, usize);
    let cs: (usize, usize);
    let rrs: (i64, i64);
    let ccs: (i64, i64);
    if r == 0 {
        rs = (rows - 1, 1);
        rrs = (rr - 1, rr);
    } else if r == rows - 1 {
        rs = (rows - 2, 0);
        rrs = (rr, rr + 1);
    } else {
        rs = (r - 1, r + 1);
        rrs = (rr, rr);
    }
    if c == 0 {
        cs = (cols - 1, 1);
        ccs = (cc - 1, cc);
    } else if c == cols - 1 {
        cs = (cols - 2, 0);
        ccs = (cc, cc + 1);
    } else {
        cs = (c - 1, c + 1);
        ccs = (cc, cc);
    }
    return vec![
        (rs.0, c, rrs.0, cc),
        (rs.1, c, rrs.1, cc),
        (r, cs.0, rr, ccs.0),
        (r, cs.1, rr, ccs.1)
    ];
}

fn get_num_covered(
    num_steps: usize,
    start: &Point,
    rows: usize,
    cols: usize,
    viable: &HashSet<PointGrid>
) -> usize {
    // let mut visited: HashMap<Point, HashSet<Point>> = HashMap::new();
    // let mut step_points: HashMap<usize, HashSet<Point>> = HashMap::new();
    let mut next_points: HashSet<Point> = HashSet::new();
    let mut update_points: HashSet<Point> = HashSet::new();
    let mut last_points: HashSet<Point> = HashSet::new();
    next_points.insert(*start);
    // step_points.insert(0, next_points.clone());

    // let num_steps = 65 + 131*4;
    let mut num0: usize = (num_steps - 1) % 2;
    for idx in 1..num_steps + 1 {
        for p in &next_points {
            // visited.insert(p, HashSet::new());
            for np in adj_points(&p, &rows, &cols)
                .iter()
                .filter(
                    |sp|
                        // !&visited.contains_key(sp) &&
                        !last_points.contains(sp) && viable.contains(&(sp.0, sp.1))
                ) {
                // println!("{:?}", np);
                update_points.insert(*np);
            }
            // for np in &update_points {
            //     visited.get_mut(&p).unwrap().insert(*np);
            // }
        }
        last_points.extend(next_points.drain());
        next_points.extend(update_points.drain());
        // step_points.insert(idx+1, next_points.clone());
        // println!("{:?}", next_points);
        num0 += if idx % 2 == num_steps % 2 && idx < num_steps + 1 { next_points.len() } else { 0 };
    }
    return num0;
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

    let mut viable: HashSet<PointGrid> = HashSet::new();
    let mut start: Point = (0, 0, 0, 0);

    for (idx, row) in all_lines.iter().enumerate() {
        for (jdx, c) in row.chars().enumerate() {
            if c == 'S' {
                start = (idx, jdx, 0, 0);
            }
            if c != '#' {
                viable.insert((idx, jdx));
            }
        }
    }

    let rows: usize = all_lines.len();
    let cols: usize = all_lines[0].len();
    let num0: usize = get_num_covered(64, &start, rows, cols, &viable);

    let xvals: Vec<usize> = vec![0, 1, 2, 3];
    let mut yvals: Vec<usize> = vec![];

    for x in xvals {
        yvals.push(get_num_covered(65 + rows * x, &start, rows, cols, &viable));
    }

    let c: usize = yvals[0];
    let a: usize = (yvals[3] - yvals[2] - (yvals[2] - yvals[1])) / 2;
    let b: usize = yvals[1] - yvals[0] - a;

    let x: usize = (26501365 - 65) / rows;

    let num1: usize = a * (x * x) + b * x + c;

    return (DAY - 1, format!("{}", num0), format!("{}", num1));
}
