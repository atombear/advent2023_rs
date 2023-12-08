use std::path::PathBuf;
use std::collections::HashMap;
use crate::utils::read_lines;

fn prime_fac(n: u64) -> HashMap<u64, u64> {
    let mut n = n;
    let mut ret: HashMap<u64, u64> = HashMap::new();
    for i in 2..n / 2 {
        if n % i == 0 {
            ret.insert(i, 0);
            while n % i == 0 {
                *ret.get_mut(&i).unwrap() += 1;
                n /= i;
            }
        }
    }

    return ret;
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input8".to_string()].iter().collect();

    let mut all_lines: Vec<String> = vec![];
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                all_lines.push(num_str);
            }
        }
    }
    let instructions: Vec<char> = all_lines[0].chars().collect::<Vec<char>>();

    let mut lmap: HashMap<String, (String, String)> = HashMap::new();
    let mut line: &String;
    let mut key: String;
    let mut l: String;
    let mut r: String;

    for idx in 2..(&all_lines).len() {
        line = &all_lines[idx];
        key = line.split('=').nth(0).unwrap().replace(' ', "");
        l = line
            .split('=')
            .nth(1)
            .unwrap()
            .split(',')
            .nth(0)
            .unwrap()
            .replace("(", "")
            .replace(" ", "");
        r = line
            .split('=')
            .nth(1)
            .unwrap()
            .split(',')
            .nth(1)
            .unwrap()
            .replace(")", "")
            .replace(" ", "");
        lmap.insert(key, (l, r));
    }

    let mut ins: char;

    let mut node: &String = &"AAA".to_string();
    let mut steps0: u64 = 0;
    while node != "ZZZ" {
        ins = instructions[(steps0 % (instructions.len() as u64)) as usize];
        if ins == 'L' {
            node = &lmap.get(node).unwrap().0;
        } else {
            node = &lmap.get(node).unwrap().1;
        }
        steps0 += 1;
    }

    let nodes: Vec<&String> = lmap
        .keys()
        .into_iter()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<&String>>();

    let mut offsets_periods: Vec<u64> = vec![];
    let mut node: &String;
    let mut steps1: u64;
    let mut finished: bool;
    for start in nodes {
        node = start;
        steps1 = 0;
        finished = false;

        while !finished {
            ins = instructions[(steps1 % (instructions.len() as u64)) as usize];
            if node.ends_with('Z') {
                offsets_periods.push(steps1);
                if offsets_periods.len() % 1 == 0 {
                    finished = true;
                }
            }
            if ins == 'L' {
                node = &lmap.get(node).unwrap().0;
            } else {
                node = &lmap.get(node).unwrap().1;
            }
            steps1 += 1;
        }
    }

    let mut lcm: HashMap<u64, u64> = HashMap::new();

    for n in &offsets_periods {
        for (k, v) in prime_fac(*n).iter() {
            if !lcm.contains_key(k) {
                lcm.insert(*k, *v);
            } else if lcm.get(k).unwrap() < v {
                lcm.insert(*k, *v);
            }
        }
    }

    let steps1: u64 = lcm
        .iter()
        .map(|(k, v)| k.pow(*v as u32))
        .product();

    return (7, format!("{}", steps0), format!("{}", steps1));
}
