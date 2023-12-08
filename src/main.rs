extern crate core;

mod utils;
mod problem20231201;
mod problem20231202;
mod problem20231203;
mod problem20231204;
mod problem20231205;
mod problem20231206;
mod problem20231207;
mod problem20231208;

fn process_answer(answers: &mut Vec<String>, idx: usize, ans0: String, ans1: String) {
    while answers.len() <= idx {
        answers.push("".to_string());
    }
    assert_eq!(answers[idx], "");
    answers[idx] = format!("{} {}", ans0, ans1);
}

fn main() {
    let mut answers: Vec<String> = vec![];

    // String, String
    for daily_fn in [
        problem20231201::problem,
        problem20231202::problem,
        problem20231203::problem,
        problem20231204::problem,
        problem20231205::problem,
        problem20231206::problem,
        problem20231207::problem,
        problem20231208::problem,
    ] {
        let (idx, ans0, ans1) = daily_fn();
        process_answer(&mut answers, idx, ans0, ans1);
    }

    for (idx, ans) in answers.iter().enumerate() {
        println!("Day {} {}", idx + 1, ans);
    }
}
