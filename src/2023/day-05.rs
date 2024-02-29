const YEAR: &'static str = "2023";
const DAY: &'static str = "05";

fn part1(_cards: &Vec<String>) -> String {
    "1".to_string()
}

fn part2(_cards: &Vec<String>) -> String {
    "2".to_string()
}

fn main() {
    let inputs = aoc::Inputs::new(YEAR, DAY);

    let sample_res = match &inputs.sample {
        Some(input) => {
            // let cards = lines_to_cards(input);
            aoc::DayResults::new(part1(input), part2(input))
        }
        None => aoc::DayResults::new(
            "<SAMPLE INPUT UNAVAILABLE>".to_string(),
            "<SAMPLE INPUT UNAVAILABLE>".to_string(),
        ),
    };
    sample_res.print("=> Sample Results:");

    let input_res = match &inputs.input {
        Some(input) => {
            // let cards = lines_to_cards(input);
            aoc::DayResults::new(part1(input), part2(input))
        }
        None => aoc::DayResults::new(
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
        ),
    };
    input_res.print("=> Actual Input Results:");
}
