use std::path::PathBuf;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::utils::read_lines;

const DAY: usize = 23;

type Point = (usize, usize);

fn right(loc: &Point) -> Point {
    let &(r, c) = loc;
    return (r, c + 1);
}
fn down(loc: &Point) -> Point {
    let &(r, c) = loc;
    return (r + 1, c);
}
fn left(loc: &Point) -> Point {
    let &(r, c) = loc;
    return (r, c - 1);
}
fn up(loc: &Point) -> Point {
    let &(r, c) = loc;
    return (r - 1, c);
}

fn next_locs(loc: &Point, hillmap: &Vec<Vec<char>>) -> Vec<Point> {
    let mut ret: Vec<Point> = vec![];

    let rows: usize = hillmap.len();
    let cols: usize = hillmap[0].len();

    let mut new_loc: Point;
    let mut r: usize;
    let mut c: usize;
    (r, c) = *loc;
    if hillmap[r][c] == '>' {
        ret.push(right(loc));
    } else if hillmap[r][c] == 'v' {
        ret.push(down(loc));
    } else if hillmap[r][c] == '<' {
        ret.push(left(loc));
    } else {
        for mv in [up, left, down, right] {
            new_loc = mv(loc);
            (r, c) = new_loc;
            if r < rows && c < cols && hillmap[r][c] != '#' {
                ret.push(new_loc);
            }
        }
    }

    return ret;
}

fn find_long_path(
    start: &Point,
    end: &Point,
    hillmap: &Vec<Vec<char>>,
    visited: &mut HashSet<Point>
) -> usize {
    let mut ret: Vec<usize> = if start == end { vec![visited.len()] } else { vec![0] };
    for new_loc in next_locs(start, hillmap) {
        if !visited.contains(&new_loc) {
            visited.insert(new_loc);
            ret.push(find_long_path(&new_loc, end, hillmap, visited));
            visited.remove(&new_loc);
        }
    }

    return *ret.iter().max().unwrap();
}

fn find_long_path_sm(
    start: &Point,
    end: &Point,
    short_map: &HashMap<Point, HashSet<(Point, usize)>>,
    visited: &mut HashSet<Point>
) -> usize {
    let mut ret: Vec<usize> = if start == end { vec![visited.len()] } else { vec![0] };
    let children = short_map.get(start).unwrap();
    for (child, dist) in children {
        if !visited.contains(&child) {
            visited.insert(*child);
            ret.push(dist + find_long_path_sm(child, end, short_map, visited));
            visited.remove(&child);
        }
    }

    return *ret.iter().max().unwrap();
}

// fn path_find(
//     start: &Point,
//     current: &Point,
//     end: &Point,
//     short_map: &mut HashMap<Point, HashSet<(Point, usize)>>,
//     flat_hillmap: &Vec<Vec<char>>,
//     visited: &mut HashSet<Point>,
//     len: usize
// ) {
//     if current == end {
//         short_map.get_mut(start).unwrap().insert((*end, len));
//     }

//     let next_points: Vec<(usize, usize)> = next_locs(start, flat_hillmap)
//         .iter()
//         .filter(|p| !visited.contains(p) || short_map.contains_key(p))
//         .map(|x| *x)
//         .collect::<Vec<Point>>();

//     if next_points.len() == 1{
//         visited.insert(*start);
//         path_find(&next_points[0], short_map, flat_hillmap, visited, len+1);
//     } else if next_points.len() > 1 {

//     }
// }

fn path_find(
    start: &Point,
    corners: &HashSet<Point>,
    visited: &mut HashSet<Point>,
    flat_hillmap: &Vec<Vec<char>>
) -> Option<(Point, usize)> {
    let mut current: Point = *start;
    let mut dist: usize = 0;
    loop {
        visited.insert(current);
        current = *next_locs(&current, flat_hillmap)
            .iter()
            .filter(|p| !visited.contains(p))
            .next()
            .unwrap_or(&(10000, 10000));
        dist += 1;
        if corners.contains(&current) {
            return Some((current, dist));
        } else if current == (10000, 10000) {
            return None;
        }
    }
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

    let mut hillmap: Vec<Vec<char>> = vec![];
    for row in all_lines.iter() {
        hillmap.push(row.chars().collect::<Vec<char>>());
    }

    let rows: usize = all_lines.len();
    let cols: usize = all_lines[0].len();

    let start: Point = (
        0,
        hillmap[0]
            .iter()
            .position(|&c| c == '.')
            .unwrap(),
    );
    let end: Point = (
        rows - 1,
        hillmap[rows - 1]
            .iter()
            .position(|&c| c == '.')
            .unwrap(),
    );

    let mut visited: HashSet<Point> = HashSet::new();
    let max_len0: usize = find_long_path(&start, &end, &hillmap, &mut visited);

    let flat_hillmap: Vec<Vec<char>> = hillmap
        .iter()
        .map(|r|
            r
                .iter()
                .map(|&c| if ['v', '<', '>'].contains(&c) { '.' } else { c })
                .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    let mut short_map: HashMap<Point, HashSet<(Point, usize)>> = HashMap::new();
    let mut corners: HashSet<Point> = HashSet::new();
    for r in 0..rows {
        for c in 0..cols {
            if next_locs(&(r, c), &hillmap).len() > 2 && hillmap[r][c] != '#' {
                corners.insert((r, c));
            }
        }
    }
    for src in [start, end] {
        visited = HashSet::new();
        if let Some((dst, steps)) = path_find(&src, &corners, &mut visited, &flat_hillmap) {
            if !short_map.contains_key(&src) {
                short_map.insert(src, HashSet::new());
            }
            short_map
                .get_mut(&src)
                .unwrap()
                .insert((dst, steps - 1));
            if !short_map.contains_key(&dst) {
                short_map.insert(dst, HashSet::new());
            }
            short_map
                .get_mut(&dst)
                .unwrap()
                .insert((src, steps - 1));
        }
    }

    for corner in &corners {
        visited = HashSet::new();
        visited.insert(*corner);
        for src in next_locs(&corner, &flat_hillmap) {
            match path_find(&src, &corners, &mut visited, &flat_hillmap) {
                Some((dst, steps)) => {
                    if !short_map.contains_key(&corner) {
                        short_map.insert(*corner, HashSet::new());
                    }
                    short_map.get_mut(corner).unwrap().insert((dst, steps));
                }
                None => {}
            }
        }
    }

    visited = HashSet::new();
    let max_len1: usize = find_long_path_sm(&start, &end, &short_map, &mut visited);

    return (DAY - 1, format!("{}", max_len0), format!("{}", max_len1));
}
