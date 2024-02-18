const YEAR: &'static str = "2023";
const DAY: &'static str = "03";

struct Schematic {}

fn input_to_schematic(_input: &Vec<String>) -> Schematic {
    Schematic {}
}

fn part1(_schematic: &Schematic) -> String {
    "1".to_string()
}

fn part2(_schematic: &Schematic) -> String {
    "2".to_string()
}

fn main() {
    let inputs = aoc::Inputs::new(YEAR, DAY);

    let sample_res = match &inputs.sample {
        Some(input) => {
            let schematic = input_to_schematic(input);
            aoc::DayResults::new(part1(&schematic), part2(&schematic))
        }
        None => aoc::DayResults::new(
            "<SAMPLE INPUT UNAVAILABLE>".to_string(),
            "<SAMPLE INPUT UNAVAILABLE>".to_string(),
        ),
    };
    sample_res.print("=> Sample Results:");

    let input_res = match &inputs.input {
        Some(input) => {
            let schematic = input_to_schematic(input);
            aoc::DayResults::new(part1(&schematic), part2(&schematic))
        }
        None => aoc::DayResults::new(
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
        ),
    };
    input_res.print("=> Actual Input Results:");
}
