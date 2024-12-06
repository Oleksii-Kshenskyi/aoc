const YEAR: &'static str = "2024";
const DAY: &'static str = "06";

use aoc::{CharField, Conv};

#[derive(Debug)]
struct AreaMap {
    matrix: CharField,
}
impl AreaMap {
    pub fn new(lines: &Vec<String>) -> Self {
        Self {
            matrix: CharField::from_lines(lines).expect("AreaMap::new(): couldn't extract char map from input!"),
        }
    }
}

fn part1(lines: &Vec<String>) -> String {
    dbg!(AreaMap::new(lines));
    "NOPE1".to_owned()
}

fn part2(lines: &Vec<String>) -> String {
    "NOPE2".to_owned()
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
