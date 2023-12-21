use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils::read_lines;

const DAY: usize = 20;

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

    let mut ffs: HashSet<String> = HashSet::new();
    let mut ffs_mem: HashMap<String, bool>;
    let mut cjs: HashSet<String> = HashSet::new();
    let mut cjs_parents: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let mut circuit: HashMap<String, HashSet<String>> = HashMap::new();
    let mut typ: char;
    let mut name: String;

    for line in all_lines.iter() {
        let line_copy = line.clone();
        let mut line_split = line_copy.splitn(2, " -> ");
        if line.contains("broadcaster") {
            circuit.insert(
                line_split.next().unwrap().to_owned(),
                line_split
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|x| x.strip_prefix(' ').unwrap_or(x).to_owned())
                    .collect::<HashSet<String>>()
            );
        } else {
            let mut typ_name = line_split.next().unwrap().chars();
            typ = typ_name.next().unwrap();
            name = typ_name.collect::<String>();
            if typ == '&' {
                cjs.insert(name.clone());
            } else {
                ffs.insert(name.clone());
            }
            circuit.insert(
                name,
                line_split
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|x| x.strip_prefix(' ').unwrap_or(x).to_owned())
                    .collect::<HashSet<String>>()
            );
        }
    }

    for parent in circuit.keys() {
        for child in circuit
            .get(parent)
            .unwrap()
            .iter()
            .filter(|c| cjs.contains(*c)) {
            if !cjs_parents.contains_key(child) {
                cjs_parents.insert(child.clone(), HashMap::new());
            }
            cjs_parents.get_mut(child).unwrap().insert(parent.clone(), false);
        }
    }
    ffs_mem = ffs
        .iter()
        .map(|s| (s.clone(), false))
        .collect::<HashMap<String, bool>>();

    let bpulses: Vec<(String, String, bool)> = circuit
        .get("broadcaster")
        .unwrap()
        .iter()
        .map(|d| ("broadcaster".to_string(), d.clone(), false))
        .collect::<Vec<(String, String, bool)>>();
    let mut pulses: Vec<(String, String, bool)>;
    let mut new_pulses: Vec<(String, String, bool)>;
    let mut temp_sig: bool;
    let mut low: usize = 0;
    let mut high: usize = 0;
    let mut all_pulses: Vec<(String, String, bool, i64)> = vec![];
    let mut result0: usize = 0;
    for idx in 0..100000 as i64 {
        low += 1;
        pulses = bpulses.clone();
        while pulses.len() > 0 {
            all_pulses.extend(
                pulses
                    .iter()
                    .filter(|(_, dst, _)| dst == "zp")
                    .map(|x| (x.0.clone(), x.1.clone(), x.2, idx))
            );
            low += pulses
                .iter()
                .filter(|(_, _, s)| !s)
                .count();
            high += pulses
                .iter()
                .filter(|(_, _, s)| *s)
                .count();
            new_pulses = vec![];
            for (src, dest, sig) in &pulses {
                if ffs.contains(dest) {
                    if !sig {
                        temp_sig = *ffs_mem.get(dest).unwrap();
                        ffs_mem.insert(dest.clone(), !temp_sig);
                        new_pulses.extend(
                            circuit
                                .get(dest)
                                .unwrap()
                                .iter()
                                .map(|d| (dest.to_owned(), d.clone(), !temp_sig))
                        );
                    }
                } else if cjs.contains(dest) {
                    cjs_parents.get_mut(dest).unwrap().insert(src.clone(), *sig);
                }
            }
            for (_, dest, _) in &pulses {
                if cjs_parents.contains_key(dest) {
                    new_pulses.extend(
                        circuit
                            .get(dest)
                            .unwrap()
                            .iter()
                            .map(|d| (
                                dest.clone(),
                                d.clone(),
                                !cjs_parents
                                    .get(dest)
                                    .unwrap()
                                    .iter()
                                    .all(|(_, v)| *v),
                            ))
                    );
                }
            }
            pulses = new_pulses;
        }
        if idx == 999 {
            result0 = low * high;
        }
    }

    let mut result1: i64 = 1;
    for p in cjs_parents.get("zp").unwrap().keys() {
        let tmp = all_pulses
            .iter()
            .filter(|(src, _, s, _)| src == p && *s)
            .map(|(_, _, _, idx)| idx)
            .map(|x| *x)
            .collect::<Vec<i64>>();
        let tmp2 = (0..tmp.len() - 1).map(|idx| tmp[idx + 1] - tmp[idx]).collect::<Vec<i64>>();
        result1 *= HashSet::<i64>
            ::from_iter(tmp2.iter().map(|x| *x))
            .iter()
            .nth(0)
            .unwrap();
    }

    return (DAY - 1, format!("{}", result0), format!("{}", result1));
}
