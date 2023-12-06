use std::path::PathBuf;
use std::collections::HashMap;
use crate::utils::read_lines;

fn get_unique<T: PartialEq>(arr: Vec<T>) -> Vec<T> {
    let mut ret: Vec<T> = vec![];

    for i in arr {
        if !ret.contains(&i) {
            ret.push(i);
        }
    }
    return ret;
}

fn get_numbers(card: &str) -> (Vec<u64>, Vec<u64>) {
    let wy_nums = card.split(':').nth(1).unwrap().split('|').into_iter().collect::<Vec<&str>>();
    let w_nums = wy_nums[0]
        .split(" ")
        .into_iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let y_nums = wy_nums[1]
        .split(" ")
        .into_iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    return (w_nums, get_unique(y_nums));
}

fn get_card_id(card: &str) -> u64 {
    return card
        .split(':')
        .nth(0)
        .unwrap()
        .split(' ')
        .into_iter()
        .rev()
        .nth(0)
        .unwrap()
        .parse::<u64>()
        .unwrap();
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input4".to_string()].iter().collect();

    let mut total_points: u64 = 0;
    let mut card_data: HashMap<u64, Vec<u64>> = HashMap::new();
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                let (w_nums, y_nums) = get_numbers(&num_str);
                let card_id: u64 = get_card_id(&num_str);

                let card_points = y_nums
                    .iter()
                    .map(|x| w_nums.contains(x))
                    .filter(|x| *x)
                    .collect::<Vec<_>>()
                    .len();
                total_points += (2_u64).pow((card_points as u32) - 1);

                card_data.insert(
                    card_id,
                    (1 + card_id..1 + card_id + (card_points as u64)).collect::<Vec<u64>>()
                );
            }
        }
    }

    let mut ids: Vec<u64> = card_data.keys().cloned().collect::<Vec<u64>>();
    ids.sort();

    let mut total_cards: u64 = 0;

    while ids.len() > 0 {
        let id = ids.pop().unwrap();
        if card_data.contains_key(&id) {
            total_cards += 1;
            ids.extend(card_data.get(&id).unwrap());
        }
    }

    return (3, format!("{}", total_points), format!("{}", total_cards));
}
