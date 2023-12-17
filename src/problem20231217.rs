use std::path::PathBuf;
use std::collections::HashSet;
use crate::utils::read_lines;

const DAY: usize = 17;

type Point = (usize, usize);
type Node = (Point, usize, char);

fn turn_left(d: char) -> char {
    return match d {
        'u' => 'l',
        'l' => 'd',
        'd' => 'r',
        'r' => 'u',
        _ => panic!("not a direction!"),
    };
}

fn turn_right(d: char) -> char {
    return match d {
        'u' => 'r',
        'l' => 'u',
        'd' => 'l',
        'r' => 'd',
        _ => panic!("not a direction!"),
    };
}

fn go_straight(d: char) -> fn((usize, usize)) -> (usize, usize) {
    return match d {
        'u' => |(r, c)| (r - 1, c),
        'l' => |(r, c)| (r, c - 1),
        'd' => |(r, c)| (r + 1, c),
        'r' => |(r, c)| (r, c + 1),
        _ => panic!("not a direction!"),
    };
}

fn get_next_nodes(n: Node, rows: usize, cols: usize) -> Vec<Node> {
    let mut ret: Vec<Node> = vec![];

    let (p, s, d) = n;

    if p == (rows - 1, cols - 1) {
        return ret;
    } else {
        if s < 3 {
            ret.push((go_straight(d)(p), s + 1, d));
        }
        ret.push((go_straight(turn_left(d))(p), 1, turn_left(d)));
        ret.push((go_straight(turn_right(d))(p), 1, turn_right(d)));

        return ret
            .iter()
            .filter(|((r, c), _, _)| r < &rows && c < &cols)
            .map(|x| *x)
            .collect();
    }
}

fn get_next_nodes_wobbly(n: Node, rows: usize, cols: usize) -> Vec<Node> {
    let mut ret: Vec<Node> = vec![];

    let (p, s, d) = n;

    if p == (rows - 1, cols - 1) {
        return ret;
    } else {
        if s < 10 {
            ret.push((go_straight(d)(p), s + 1, d));
        }
        if s >= 4 {
            ret.push((go_straight(turn_left(d))(p), 1, turn_left(d)));
            ret.push((go_straight(turn_right(d))(p), 1, turn_right(d)));
        }

        return ret
            .iter()
            .filter(|((r, c), _, _)| r < &rows && c < &cols)
            .map(|x| *x)
            .collect();
    }
}

fn timed_bfs(
    heat_loss: &Vec<Vec<u64>>,
    gnn: fn(n: Node, rows: usize, cols: usize) -> Vec<Node>
) -> u64 {
    let rows: usize = heat_loss.len();
    let cols: usize = heat_loss[0].len();

    let mut next_nodes: Vec<(u64, Node)> = vec![
        (heat_loss[1][0], ((1, 0), 1, 'd')),
        (heat_loss[0][1], ((0, 1), 1, 'r'))
    ];
    next_nodes.sort();

    let mut t: u64 = 0;

    let mut visited: HashSet<Node> = HashSet::new();

    loop {
        while next_nodes.len() > 0 && next_nodes[0].0 == t {
            let (_, node) = next_nodes[0];
            next_nodes.remove(0);

            if node.0 == (rows - 1, cols - 1) {
                return t;
            }

            if !visited.contains(&node) {
                for new_node in gnn(node, rows, cols) {
                    next_nodes.push((t + heat_loss[new_node.0.0][new_node.0.1], new_node));
                    next_nodes.sort();
                }
            }
            visited.insert(node);
        }
        t += 1;
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

    let heat_loss: Vec<Vec<u64>> = all_lines
        .iter()
        .map(|l|
            l
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        )
        .collect::<Vec<Vec<u64>>>();

    return (
        DAY - 1,
        format!("{}", timed_bfs(&heat_loss, get_next_nodes)),
        format!("{}", timed_bfs(&heat_loss, get_next_nodes_wobbly)),
    );
}
