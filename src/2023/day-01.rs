use std::collections::HashMap;

const YEAR: &'static str = "2023";
const DAY: &'static str = "01";
fn get_transform() -> HashMap<String, String> {
    HashMap::from([
        ("1".to_owned(), "1".to_owned()),
        ("2".to_owned(), "2".to_owned()),
        ("3".to_owned(), "3".to_owned()),
        ("4".to_owned(), "4".to_owned()),
        ("5".to_owned(), "5".to_owned()),
        ("6".to_owned(), "6".to_owned()),
        ("7".to_owned(), "7".to_owned()),
        ("8".to_owned(), "8".to_owned()),
        ("9".to_owned(), "9".to_owned()),
        ("one".to_owned(), "1".to_owned()),
        ("two".to_owned(), "2".to_owned()),
        ("three".to_owned(), "3".to_owned()),
        ("four".to_owned(), "4".to_owned()),
        ("five".to_owned(), "5".to_owned()),
        ("six".to_owned(), "6".to_owned()),
        ("seven".to_owned(), "7".to_owned()),
        ("eight".to_owned(), "8".to_owned()),
        ("nine".to_owned(), "9".to_owned()),
    ])
}

fn part1(input: &Vec<String>) -> String {
    let mut numbers: Vec<u16> = vec![];
    for line in input {
        let mut res: String = "".to_owned();
        res.push(line.chars().find(|c| c.is_numeric()).unwrap());
        res.push(line.chars().rev().find(|c| c.is_numeric()).unwrap());
        numbers.push(res.parse().unwrap());
    }
    numbers.iter().sum::<u16>().to_string()
}

fn part2(input: &Vec<String>) -> String {
    let mut numbers: Vec<u32> = vec![];
    let mut first_key_index: Option<usize> = None;
    let mut last_key_index: Option<usize> = None;
    let mut first_key = "";
    let mut last_key = "";
    let tf = get_transform();

    for line in input {
        for key in tf.keys() {
            if let Some(cur_index) = line.find(key) {
                if first_key_index.is_none() || cur_index < first_key_index.unwrap() {
                    first_key_index = Some(cur_index);
                    first_key = key;
                }
            }
            if let Some(cur_index) = line.rfind(key) {
                if last_key_index.is_none() || cur_index >= last_key_index.unwrap() {
                    last_key_index = Some(cur_index);
                    last_key = key;
                }
            }
        }
        numbers.push(
            format!(
                "{}{}",
                tf.get(first_key).unwrap(),
                tf.get(last_key).unwrap(),
            )
            .parse::<u32>()
            .unwrap(),
        );
        first_key_index = None;
        last_key_index = None;
        first_key = "";
        last_key = "";
    }
    numbers.iter().sum::<u32>().to_string()
}

fn main() {
    let inputs = aoc::Inputs::new(YEAR, DAY);

    let sample_res = match &inputs.sample {
        Some(input) => aoc::DayResults::new(part1(input), part2(input)),
        None => aoc::DayResults::new(
            "<SAMPLE INPUT UNAVAILABLE>".to_string(),
            "<SAMPLE INPUT UNAVAILABLE>".to_string(),
        ),
    };
    sample_res.print("=> Sample Results:");

    let input_res = match &inputs.input {
        Some(input) => aoc::DayResults::new(part1(input), part2(input)),
        None => aoc::DayResults::new(
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
        ),
    };
    input_res.print("=> Actual Input Results:");
}
