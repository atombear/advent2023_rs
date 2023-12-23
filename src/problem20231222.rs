use std::cmp::max;
use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils::read_lines;

const DAY: usize = 22;

type Point2D = (usize, usize);
type Point = (usize, usize, usize);
type Brick = (Point, Point);

fn get_line(b: &Brick) -> Vec<Point> {
    let &((x0, y0, z0), (x1, y1, z1)) = b;
    let mut ret: Vec<Point> = vec![];
    if x0 != x1 {
        for x in x0..x1 + 1 {
            ret.push((x, y0, z0));
        }
    } else if y0 != y1 {
        for y in y0..y1 + 1 {
            ret.push((x0, y, z0));
        }
    } else {
        for z in z0..z1 + 1 {
            ret.push((x0, y0, z));
        }
    }

    return ret;
}

// fn overlap(b0: &Brick, b1: &Brick) -> bool {
//     let line0 = get_line(b0);
//     for s1 in get_line(b1) {
//         if line0.contains(&s1) {
//             return true;
//         }
//     }
//     return false;
// }

fn overlap2d(b0: &Brick, b1: &Brick) -> bool {
    let line0: HashSet<(usize, usize)> = get_line(b0)
        .iter()
        .map(|p| (p.0, p.1))
        .collect::<HashSet<Point2D>>();
    for s1 in get_line(b1) {
        if line0.contains(&(s1.0, s1.1)) {
            return true;
        }
    }
    return false;
}

fn get_fallen_bricks(bricks: &mut Vec<Brick>) -> (Vec<Brick>, usize) {
    let mut fallen_bricks: Vec<Brick> = vec![];
    let mut max_heights: HashMap<Point2D, usize> = HashMap::new();
    let mut max_height: usize;
    let mut num_moved: usize = 0;
    for brick in bricks.drain(0..bricks.len()) {
        let ((x0, y0, z0), (x1, y1, z1)) = brick;
        assert!(x0 <= x1);
        assert!(y0 <= y1);
        assert!(z0 <= z1);
        max_height = 0;
        if x0 == x1 {
            for y in y0..y1 + 1 {
                max_height = max(max_height, *max_heights.get(&(x0, y)).unwrap_or(&0));
            }
            for y in y0..y1 + 1 {
                max_heights.insert((x0, y), 1 + max_height + (z1 - z0));
            }
        } else if y0 == y1 {
            for x in x0..x1 + 1 {
                max_height = max(max_height, *max_heights.get(&(x, y0)).unwrap_or(&0));
            }
            for x in x0..x1 + 1 {
                max_heights.insert((x, y0), 1 + max_height + (z1 - z0));
            }
        } else {
            panic!("what brick is this?!");
        }
        if max_height + 1 != z0 {
            num_moved += 1;
        }
        fallen_bricks.push(((x0, y0, max_height + 1), (x1, y1, max_height + 1 + (z1 - z0))));
    }
    return (fallen_bricks, num_moved);
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

    let mut bricks: Vec<Brick> = vec![];
    let mut temp_bricks: Vec<Point> = vec![];
    let mut brick_vec: Vec<usize>;
    for line in all_lines {
        for brick_str in line.splitn(2, '~') {
            brick_vec = brick_str
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            temp_bricks.insert(0, (brick_vec[0], brick_vec[1], brick_vec[2]));
        }
        bricks.push((temp_bricks.pop().unwrap(), temp_bricks.pop().unwrap()));
    }
    bricks.sort_by_key(|b| b.0.2);

    let fallen_bricks: Vec<Brick> = get_fallen_bricks(&mut bricks.clone()).0;

    let mut bricks_by_lower_height: HashMap<usize, HashSet<Brick>> = HashMap::new();
    for b in &fallen_bricks {
        if !bricks_by_lower_height.contains_key(&b.0.2) {
            bricks_by_lower_height.insert(b.0.2, HashSet::new());
        }
        bricks_by_lower_height.get_mut(&b.0.2).unwrap().insert(*b);
    }

    let mut bricks_by_upper_height: HashMap<usize, HashSet<Brick>> = HashMap::new();
    for b in &fallen_bricks {
        if !bricks_by_upper_height.contains_key(&b.1.2) {
            bricks_by_upper_height.insert(b.1.2, HashSet::new());
        }
        bricks_by_upper_height.get_mut(&b.1.2).unwrap().insert(*b);
    }

    let mut can_remove: bool;
    let mut num_remove: usize = 0;
    let mut removable_bricks: HashSet<Brick> = HashSet::new();

    // check each brick.
    for b in &fallen_bricks {
        can_remove = true;
        // find bricks resting on b.
        for bu in bricks_by_lower_height
            .get(&(b.1.2 + 1))
            .unwrap_or(&HashSet::<Brick>::new())
            .iter()
            .filter(|x| overlap2d(x, &b)) {
            // find bricks that the resting bricks are resting on.
            // each upper brick should must have at least one brick below it to rest on
            // that is not the brick in question.
            if
                bricks_by_upper_height
                    .get(&(bu.0.2 - 1))
                    .unwrap_or(&HashSet::<Brick>::new())
                    .iter()
                    .filter(|x| overlap2d(x, bu) && *x != b)
                    .count() == 0
            {
                can_remove &= false;
            }
        }
        if can_remove {
            removable_bricks.insert(*b);
            num_remove += 1;
        }
    }

    let mut total_num_moved: usize = 0;
    let mut new_bricks: Vec<Brick>;
    let unmovable_bricks: Vec<Brick> = fallen_bricks
        .iter()
        .filter(|x| !removable_bricks.contains(x))
        .map(|x| *x)
        .collect::<Vec<Brick>>();
    for b in unmovable_bricks {
        new_bricks = fallen_bricks.clone();
        new_bricks.remove(
            new_bricks
                .iter()
                .position(|&x| x == b)
                .unwrap()
        );

        total_num_moved += get_fallen_bricks(&mut new_bricks.clone()).1;
    }

    return (DAY - 1, format!("{}", num_remove), format!("{}", total_num_moved));
}
