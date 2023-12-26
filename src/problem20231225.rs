use std::path::PathBuf;
use std::collections::{ HashMap, HashSet };
use ndarray::{ Array2, s, ArrayView1 };
use ndarray_linalg::{ Eigh, UPLO };
extern crate blas_src;

use crate::utils::read_lines;

const DAY: usize = 25;

type Node = String;
type Edge = (Node, Node);
type Graph = HashMap<Node, HashSet<Node>>;

fn get_edges(graph: &Graph) -> HashSet<Edge> {
    let mut ret: HashSet<Edge> = HashSet::new();

    for parent in graph.keys() {
        for child in graph.get(parent).unwrap() {
            if parent < child {
                ret.insert((parent.clone(), child.clone()));
            } else {
                ret.insert((child.clone(), parent.clone()));
            }
        }
    }

    return ret;
}

// fn traverse_graph(enter: &Node, graph: &Graph, visited: &mut HashSet<Node>) {
//     visited.insert(enter.clone());
//     for child in graph
//         .get(enter)
//         .unwrap()
//         .iter()
//         .filter(|n| !visited.contains(*n))
//         .collect::<Vec<&String>>() {
//         traverse_graph(child, graph, visited);
//     }
// }

// fn get_num_graphs(graph: &Graph) -> usize {
//     let mut visited: HashSet<Node> = HashSet::new();

//     let mut num_graphs: usize = 0;
//     let mut enter: Node;

//     while visited.len() < graph.keys().len() {
//         num_graphs += 1;
//         enter = graph
//             .keys()
//             .into_iter()
//             .filter(|n| !visited.contains(*n))
//             .nth(0)
//             .unwrap()
//             .clone();
//         traverse_graph(&enter, graph, &mut visited);
//     }

//     return num_graphs;
// }

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

    let mut graph: Graph = HashMap::new();
    let mut key: String;
    let mut children: Vec<String>;
    for line in all_lines {
        let mut line_split = line.splitn(2, ": ");
        key = line_split.next().unwrap().to_string();
        children = line_split
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for child in children {
            if !graph.contains_key(&key) {
                graph.insert(key.clone(), HashSet::new());
            }
            graph.get_mut(&key).unwrap().insert(child.clone());
            if !graph.contains_key(&child) {
                graph.insert(child.clone(), HashSet::new());
            }
            graph.get_mut(&child).unwrap().insert(key.clone());
        }
    }

    let nodes_vec: Vec<&Node> = graph.keys().collect::<Vec<&Node>>();
    let num_nodes: usize = nodes_vec.len();
    let edges: HashSet<Edge> = get_edges(&graph);

    let mut adj: Array2<f64> = Array2::zeros((num_nodes, num_nodes));
    let mut deg: Array2<f64> = Array2::zeros((num_nodes, num_nodes));

    let mut idx: usize;
    let mut jdx: usize;
    for (n0, n1) in edges {
        idx = nodes_vec
            .iter()
            .position(|&n| *n == n0)
            .unwrap();
        jdx = nodes_vec
            .iter()
            .position(|&n| *n == n1)
            .unwrap();
        adj[(idx, jdx)] = 1.0;
        adj[(jdx, idx)] = 1.0;
    }
    for parent in graph.keys() {
        idx = nodes_vec
            .iter()
            .position(|&n| n == parent)
            .unwrap();
        deg[(idx, idx)] = graph.get(parent).unwrap().len() as f64;
    }

    let lap: Array2<f64> = deg - adj;
    let (_, eigvecs) = lap.eigh(UPLO::Lower).unwrap();

    let fiedler: ArrayView1<f64> = eigvecs.slice(s![.., 1]);
    let num0: usize = fiedler
        .iter()
        .filter(|&x| *x < 0.0)
        .count();
    let num1: usize = num_nodes - num0;

    return (DAY - 1, format!("{}", num0 * num1), format!("{}", 0));
}
