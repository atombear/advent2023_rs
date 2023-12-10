use std::path::PathBuf;
use crate::utils::read_lines;
use std::collections::HashSet;

const DAY: usize = 10;

type Point = (usize, usize);

fn get_next(rc: Point, maze: &Vec<Vec<char>>, visited: &Point) -> Point {
    let (r, c) = rc;
    if
        r > 0 &&
        !(visited == &(r - 1, c)) &&
        "|F7S".contains(maze[r - 1][c]) &&
        "S|JL".contains(maze[r][c])
    {
        return (r - 1, c);
    } else if
        c + 1 < maze[0].len() &&
        !(visited == &(r, c + 1)) &&
        "J-7S".contains(maze[r][c + 1]) &&
        "S-FL".contains(maze[r][c])
    {
        return (r, c + 1);
    } else if
        r + 1 < maze.len() &&
        !(visited == &(r + 1, c)) &&
        "|JLS".contains(maze[r + 1][c]) &&
        "S|7F".contains(maze[r][c])
    {
        return (r + 1, c);
    } else if
        c > 0 &&
        !(visited == &(r, c - 1)) &&
        "F-LS".contains(maze[r][c - 1]) &&
        "S-7J".contains(maze[r][c])
    {
        return (r, c - 1);
    } else {
        panic!(
            "no escape found at {:?} from {:?}, {} {}",
            rc,
            visited,
            maze[rc.0][rc.1],
            maze[visited.0][visited.1]
        );
    }
}

fn quadrant(r: i64, c: i64) -> usize {
    let z: i64 = 0;
    if r > z && c <= z {
        return 0;
    } else if r >= z && c > z {
        return 1;
    } else if r < z && c >= z {
        return 2;
    } else if r <= z && c < z {
        return 3;
    } else {
        panic!("No quadrant for {} {}", r, c);
    }
}

fn in_loop(p: &Point, visited: &Vec<Point>) -> bool {
    let mut quad: usize;
    let mut last_quad: usize;

    last_quad = quadrant(
        (visited[0].0 as i64) - (p.0 as i64),
        (visited[0].1 as i64) - (p.1 as i64)
    );

    let mut quad_path: Vec<usize> = vec![last_quad];
    for (r, c) in visited.iter().skip(1) {
        quad = quadrant((*r as i64) - (p.0 as i64), (*c as i64) - (p.1 as i64));
        if quad != last_quad {
            quad_path.push(quad);
        }
        last_quad = quad;
    }

    let mut new_quad_path: Vec<usize> = vec![];
    let mut len: usize;
    for q in quad_path {
        new_quad_path.push(q);
        len = new_quad_path.len();
        if len >= 3 && new_quad_path[len - 1] == new_quad_path[len - 3] {
            new_quad_path.truncate(len - 2);
        }
    }

    return [0, 1, 2, 3]
        .map(|x| new_quad_path.contains(&x))
        .iter()
        .all(|x| *x);
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

    let maze: Vec<Vec<char>> = all_lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let sr: usize = all_lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.contains('S'))
        .nth(0)
        .unwrap().0;
    let sc: usize = all_lines[sr].find('S').unwrap();

    let mut cur_r: usize = sr;
    let mut cur_c: usize = sc;
    let mut lr: usize;
    let mut lc: usize;
    let mut v_vec: Vec<Point> = vec![];
    let mut v_set: HashSet<Point> = HashSet::new();
    let mut temp: Point;

    v_set.insert((cur_r, cur_c));
    v_vec.push((cur_r, cur_c));
    lr = cur_r;
    lc = cur_c;
    (cur_r, cur_c) = get_next((cur_r, cur_c), &maze, &(cur_r, cur_c));
    while (cur_r, cur_c) != (sr, sc) {
        v_set.insert((cur_r, cur_c));
        v_vec.push((cur_r, cur_c));
        temp = (cur_r, cur_c);
        (cur_r, cur_c) = get_next((cur_r, cur_c), &maze, &(lr, lc));
        (lr, lc) = temp;
    }
    v_vec.push(v_vec[0]);

    let mut interior: usize = 0;
    for r in 0..maze.len() {
        for c in 0..maze[0].len() {
            if !v_set.contains(&(r, c)) {
                if in_loop(&(r, c), &v_vec) {
                    interior += 1;
                }
            }
        }
    }

    return (DAY - 1, format!("{}", v_set.len() / 2), format!("{}", interior));
}
