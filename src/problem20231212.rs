use std::path::PathBuf;
use std::collections::HashMap;
use crate::utils::read_lines;

const DAY: usize = 12;

fn get_ways(
    rec: &Vec<char>,
    pat: &Vec<u64>,
    cnt: i64,
    rdx: usize,
    pdx: usize,
    cache: &mut HashMap<(i64, usize, usize), u64>
) -> u64 {
    if rdx == rec.len() {
        if cnt == 0 && pdx == pat.len() - 1 {
            return 1;
        } else {
            return 0;
        }
    }

    let key: (i64, usize, usize) = (cnt, rdx, pdx);

    if !cache.contains_key(&key) {
        let val: u64 = if rec[rdx] == '#' {
            if
                cnt == 0 ||
                rdx + ((cnt - 1) as usize) >= rec.len() ||
                (0..cnt as usize).map(|idx| rec[rdx + idx] == '.').any(|x| x)
            {
                0
            } else {
                get_ways(rec, pat, 0, rdx + (cnt as usize), pdx, cache)
            }
        } else if rec[rdx] == '.' {
            if cnt == 0 && pdx + 1 < pat.len() {
                get_ways(rec, pat, pat[pdx + 1] as i64, rdx + 1, pdx + 1, cache)
            } else {
                get_ways(rec, pat, cnt, rdx + 1, pdx, cache)
            }
        } else {
            let dot: u64 = if cnt == 0 && pdx + 1 < pat.len() {
                get_ways(rec, pat, pat[pdx + 1] as i64, rdx + 1, pdx + 1, cache)
            } else {
                get_ways(rec, pat, cnt, rdx + 1, pdx, cache)
            };

            let hashtag: u64 = if
                cnt == 0 ||
                rdx + ((cnt - 1) as usize) >= rec.len() ||
                (0..cnt as usize).map(|idx| rec[rdx + idx] == '.').any(|x| x)
            {
                0
            } else {
                get_ways(rec, pat, 0, rdx + (cnt as usize), pdx, cache)
            };

            dot + hashtag
        };
        cache.insert(key, val);
    }
    return *cache.get(&key).unwrap();
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

    let pats: Vec<Vec<u64>> = all_lines
        .iter()
        .map(|l|
            l
                .split(" ")
                .nth(1)
                .unwrap()
                .split(',')
                .map(|c| c.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        )
        .collect::<Vec<Vec<u64>>>();
    let recs: Vec<Vec<char>> = all_lines
        .iter()
        .map(|l| l.split(' ').nth(0).unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut cache: HashMap<(i64, usize, usize), u64>;

    let mut cnt0: u64 = 0;
    for (r, p) in recs.iter().zip(&pats) {
        cache = HashMap::new();
        cnt0 += get_ways(r, p, p[0] as i64, 0, 0, &mut cache);
    }

    let mut cnt1: u64 = 0;
    let mut jr: Vec<char>;
    let mut pr: Vec<u64>;
    for (r, p) in recs.iter().zip(&pats) {
        jr = vec![];
        pr = vec![];
        for _ in 0..5 {
            jr.extend(r);
            jr.push('?');
            pr.extend(p);
        }
        jr.pop();
        cache = HashMap::new();
        cnt1 += get_ways(&jr, &pr, p[0] as i64, 0, 0, &mut cache);
    }

    return (DAY - 1, format!("{}", cnt0), format!("{}", cnt1));
}
