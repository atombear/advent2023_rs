use std::path::PathBuf;
use std::collections::HashMap;
use crate::utils::read_lines;

const DAY: usize = 19;

type XMAS = (u64, u64, u64, u64);

fn get_xmas(xmas: XMAS, x: char) -> u64 {
    match x {
        'x' => xmas.0,
        'm' => xmas.1,
        'a' => xmas.2,
        's' => xmas.3,
        _ => panic!("UNKNOWN field!"),
    }
}

fn make_ltgt_rule(x: char, cmp_char: char, val: u64) -> impl Fn(XMAS) -> bool {
    return Box::new(move |xmas: (u64, u64, u64, u64)| (
        if cmp_char == '>' {
            get_xmas(xmas, x) > val
        } else {
            get_xmas(xmas, x) < val
        }
    ));
}

fn process_rule(rule_str: &str) -> impl Fn(XMAS) -> bool {
    let x: char;
    let val: u64;
    let cmp_char: char = if rule_str.contains('<') { '<' } else { '>' };
    let mut split = rule_str.splitn(2, cmp_char);
    x = split.next().unwrap().chars().nth(0).unwrap();
    val = split.next().unwrap().parse::<u64>().unwrap();
    return make_ltgt_rule(x, cmp_char, val);
}

type XmasRange = Vec<(u64, u64)>;
fn xmas_size(xr: &XmasRange) -> u64 {
    let mut ret: u64 = 1;
    for (l, u) in xr {
        ret *= 1 + u - l;
    }
    return ret;
}

fn apply_rule(xr: XmasRange, rule: &String) -> (XmasRange, XmasRange) {
    let mut new: XmasRange = xr.clone();
    let mut rest: XmasRange = xr.clone();

    let mut iter = rule.chars();
    let idx: usize = match iter.next() {
        Some('x') => 0,
        Some('m') => 1,
        Some('a') => 2,
        Some('s') => 3,
        _ => panic!("WHAT LETTER"),
    };
    let gt: bool = iter.next().unwrap() == '>';
    let val: u64 = iter.collect::<String>().parse::<u64>().unwrap();

    if gt {
        let (l, u) = xr[idx];
        if val < l {
            rest[idx] = (1, 0);
        } else if u <= val {
            new[idx] = (1, 0);
        } else {
            new[idx] = (val + 1, u);
            rest[idx] = (l, val);
        }
    } else {
        let (l, u) = xr[idx];
        if u < val {
            rest[idx] = (1, 0);
        } else if val <= l {
            new[idx] = (1, 0);
        } else {
            new[idx] = (l, val - 1);
            rest[idx] = (val, u);
        }
    }

    return (new, rest);
}

fn get_sum_by_split(str_rules: &HashMap<String, Vec<(String, String)>>) -> u64 {
    let mut to_place: Vec<(XmasRange, String)> = vec![(
        vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
        "in".to_owned(),
    )];
    let mut next_place: Vec<(XmasRange, String)>;
    let mut ret: u64 = 0;

    while to_place.len() > 0 {
        next_place = vec![];
        for (xmas_range, dest) in to_place {
            let mut rest: XmasRange = xmas_range;
            let mut new: XmasRange;
            if dest == "R" {
            } else if dest == "A" {
                ret += xmas_size(&rest);
            } else {
                for (rule, nxt_dst) in str_rules.get(&dest).unwrap() {
                    if rule.len() == 0 {
                        next_place.push((rest.clone(), nxt_dst.clone()));
                    } else {
                        (new, rest) = apply_rule(rest, rule);
                        if xmas_size(&new) > 0 {
                            next_place.push((new, nxt_dst.clone()));
                        }
                    }
                }
            }
        }
        to_place = next_place;
    }

    return ret;
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

    let mut ratings: Vec<XMAS> = vec![];
    let mut process_ratings: bool = false;
    let mut xmas_vals: Vec<u64>;

    let mut rules: HashMap<String, Vec<(Box<dyn Fn(XMAS) -> bool>, String)>> = HashMap::new();
    let mut str_rules: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut dest: String;
    let mut ins_str: String;
    let mut next_dest: String;

    for line in all_lines {
        if line.len() == 0 {
            process_ratings = true;
            continue;
        }

        if process_ratings {
            xmas_vals = line
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|s| s.split('=').nth(1).unwrap().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            ratings.push((xmas_vals[0], xmas_vals[1], xmas_vals[2], xmas_vals[3]));
        } else {
            let mut split = line.splitn(2, '{');
            dest = split.next().unwrap().to_owned();
            let key = dest.clone();
            rules.insert(dest.clone(), vec![]);
            str_rules.insert(dest.clone(), vec![]);
            for rule_str in split.next().unwrap().strip_suffix('}').unwrap().split(',') {
                if rule_str.contains(':') {
                    let mut rule_split = rule_str.splitn(2, ':');
                    ins_str = rule_split.next().unwrap().to_owned();
                    next_dest = rule_split.next().unwrap().to_owned();
                    rules
                        .get_mut(&key)
                        .unwrap()
                        .push((Box::new(process_rule(ins_str.as_str())), next_dest.clone()));
                    str_rules.get_mut(&key).unwrap().push((ins_str.clone(), next_dest.clone()));
                } else {
                    next_dest = rule_str.to_owned();
                    rules
                        .get_mut(&key)
                        .unwrap()
                        .push((Box::new(|_: XMAS| true), next_dest.clone()));
                    str_rules.get_mut(&key).unwrap().push(("".to_owned(), next_dest.clone()));
                }
            }
        }
    }
    let mut total_sum0: u64 = 0;
    for r in ratings {
        let mut state: String = "in".to_string();
        let mut term: bool = false;
        while !term {
            for (rule, dest) in rules.get(&state).unwrap() {
                if rule(r) {
                    state = dest.clone();
                    break;
                }
            }
            if state == "A" || state == "R" {
                if state == "A" {
                    total_sum0 += r.0 + r.1 + r.2 + r.3;
                }
                term = true;
            }
        }
    }

    let total_sum1: u64 = get_sum_by_split(&str_rules);

    return (DAY - 1, format!("{}", total_sum0), format!("{}", total_sum1));
}
