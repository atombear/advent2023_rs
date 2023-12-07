use std::path::PathBuf;
use std::collections::HashSet;
use crate::utils::read_lines;

const CARDORDER: &str = "23456789TJQKA";
const CARDORDERJ: &str = "J23456789TQKA";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Hand {
    HighCard(Vec<u64>),
    OnePair(Vec<u64>),
    TwoPair(Vec<u64>),
    Three(Vec<u64>),
    FullHouse(Vec<u64>),
    Four(Vec<u64>),
    Five(Vec<u64>),
}

fn classify_hand(hand: &Vec<u64>) -> Hand {
    let card_set: HashSet<u64> = HashSet::from_iter(hand.iter().map(|x| *x));
    if card_set.len() == 1 {
        return Hand::Five(hand.clone());
    } else if card_set.len() == 2 {
        let num0: usize = hand
            .iter()
            .filter(|x| **x == hand[0])
            .count();
        if num0 == 1 || num0 == 4 {
            return Hand::Four(hand.clone());
        } else {
            return Hand::FullHouse(hand.clone());
        }
    } else if card_set.len() == 3 {
        if
            hand
                .iter()
                .map(|x|
                    hand
                        .iter()
                        .filter(|y| x == *y)
                        .count()
                )
                .any(|x| x == 3)
        {
            return Hand::Three(hand.clone());
        } else {
            return Hand::TwoPair(hand.clone());
        }
    } else if card_set.len() == 4 {
        return Hand::OnePair(hand.clone());
    } else {
        return Hand::HighCard(hand.clone());
    }
}

fn classify_hand_j(hand: &Vec<u64>, og_hand: &Vec<u64>) -> Hand {
    let card_set: HashSet<u64> = HashSet::from_iter(hand.iter().map(|x| *x));
    if card_set.len() == 1 {
        return Hand::Five(og_hand.clone());
    } else if card_set.len() == 2 {
        let num0: usize = hand
            .iter()
            .filter(|x| **x == hand[0])
            .count();
        if num0 == 1 || num0 == 4 {
            return Hand::Four(og_hand.clone());
        } else {
            return Hand::FullHouse(og_hand.clone());
        }
    } else if card_set.len() == 3 {
        if
            hand
                .iter()
                .map(|x|
                    hand
                        .iter()
                        .filter(|y| x == *y)
                        .count()
                )
                .any(|x| x == 3)
        {
            return Hand::Three(og_hand.clone());
        } else {
            return Hand::TwoPair(og_hand.clone());
        }
    } else if card_set.len() == 4 {
        return Hand::OnePair(og_hand.clone());
    } else {
        return Hand::HighCard(og_hand.clone());
    }
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input7".to_string()].iter().collect();

    let mut all_lines: Vec<String> = vec![];
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                all_lines.push(num_str);
            }
        }
    }

    let mut hand_bid_type: Vec<(Hand, String, u64)> = vec![];
    for line in &all_lines {
        let str_hand = line.split(' ').nth(0).unwrap().to_string();
        let hand: Vec<u64> = line
            .split(' ')
            .nth(0)
            .unwrap()
            .chars()
            .map(|x| CARDORDER.find(x).unwrap() as u64)
            .collect::<Vec<u64>>();
        let bid: u64 = line.split(' ').nth(1).unwrap().parse::<u64>().unwrap();
        let typ: Hand = classify_hand(&hand);
        hand_bid_type.push((typ, str_hand, bid));
    }
    hand_bid_type.sort();
    let score: u64 = hand_bid_type
        .iter()
        .enumerate()
        .map(|(idx, (_, _, b))| ((idx as u64) + 1) * b)
        .sum::<u64>();

    let mut hand_bid_type_j: Vec<(Hand, String, u64)> = vec![];
    for line in all_lines {
        let mut hands: Vec<Hand> = vec![];

        let og_str_hand: String = line.split(' ').nth(0).unwrap().to_string();
        let og_hand: Vec<u64> = og_str_hand
            .chars()
            .map(|x| CARDORDERJ.find(x).unwrap() as u64)
            .collect::<Vec<u64>>();

        let bid: u64 = line.split(' ').nth(1).unwrap().parse::<u64>().unwrap();
        for c in CARDORDER.chars() {
            let str_hand = line
                .split(' ')
                .nth(0)
                .unwrap()
                .to_string()
                .replace('J', format!("{}", c).as_str());
            let hand: Vec<u64> = str_hand
                .chars()
                .map(|x| CARDORDER.find(x).unwrap() as u64)
                .collect::<Vec<u64>>();
            let typ: Hand = classify_hand_j(&hand, &og_hand);
            hands.push(typ);
        }

        hand_bid_type_j.push((hands.iter().max().unwrap().clone(), og_str_hand, bid));
    }
    hand_bid_type_j.sort();
    let score_j: u64 = hand_bid_type_j
        .iter()
        .enumerate()
        .map(|(idx, (_, _, b))| ((idx as u64) + 1) * b)
        .sum::<u64>();

    return (6, format!("{}", score), format!("{}", score_j));
}
