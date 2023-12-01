use std::path::PathBuf;

use crate::utils::read_lines;

const DIGITS: &str = "0123456789";
const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_first_digit(word: &String) -> u64 {
    for c in word.chars().into_iter() {
        if DIGITS.contains(c) {
            return DIGITS.find(c).unwrap() as u64;
        }
    }
    return 0;
}

fn get_number_from_word(num_str: &String) -> u64 {
    let d0: u64 = find_first_digit(&num_str);
    let d1: u64 = find_first_digit(&num_str.chars().into_iter().rev().collect());
    return 10 * d0 + d1;
}

fn starts_with(char_vec: &Vec<char>, w: &str) -> bool {
    let mut found: bool = true;
    for (c0, c1) in char_vec.iter().zip((*w).chars()) {
        if *c0 != c1 {
            found = false;
        }
    }
    return found;
}

fn remove_n(char_vec: &mut Vec<char>, n: usize) {
    for _ in 0..n {
        char_vec.remove(0);
    }
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input1".to_string()].iter().collect();

    let mut total0: u64 = 0;
    let mut total1: u64 = 0;
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                total0 += get_number_from_word(&num_str);

                let mut char_vec: Vec<char> = num_str.to_owned().chars().into_iter().collect();

                let mut cnt: usize = 0;
                while !DIGITS.contains(char_vec[0]) {
                    if cnt > 0 {
                        char_vec.remove(0);
                    }
                    cnt += 1;
                    for (idx, w) in WORDS.iter().enumerate() {
                        if starts_with(&char_vec, w) {
                            remove_n(&mut char_vec, w.len());
                            char_vec.insert(
                                0,
                                DIGITS.chars()
                                    .nth(idx + 1)
                                    .unwrap()
                            );
                        }
                    }
                }

                char_vec.reverse();
                cnt = 0;
                while !DIGITS.contains(char_vec[0]) {
                    if cnt > 0 {
                        char_vec.remove(0);
                    }
                    cnt += 1;
                    for (idx, w) in WORDS.iter().enumerate() {
                        if starts_with(&char_vec, (*w).chars().rev().collect::<String>().as_str()) {
                            remove_n(&mut char_vec, w.len());
                            char_vec.insert(
                                0,
                                DIGITS.chars()
                                    .nth(idx + 1)
                                    .unwrap()
                            );
                        }
                    }
                }

                let num_str_repl: String = char_vec.iter().rev().collect::<String>();
                total1 += get_number_from_word(&num_str_repl);
            }
        }
    }

    return (0, format!("{}", total0), format!("{}", total1));
}
