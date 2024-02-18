use aoc::CharField;

const YEAR: &'static str = "2023";
const DAY: &'static str = "03";

struct Schematic {
    pub field: CharField,
}

impl Schematic {
    pub fn new(input: &Vec<String>) -> Self {
        Self {
            field: CharField::from_lines(input).unwrap(),
        }
    }
}

fn part1(schematic: &Schematic) -> String {
    format!(
        "Char(3,8): {}, Char(9,7): {};",
        schematic.field.get(3, 8).unwrap(),
        schematic.field.get(9, 7).unwrap(),
    )
}

fn part2(_schematic: &Schematic) -> String {
    "2".to_string()
}

fn main() {
    let inputs = aoc::Inputs::new(YEAR, DAY);

    let sample_res = match &inputs.sample {
        Some(input) => {
            let schematic = Schematic::new(input);
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
            let schematic = Schematic::new(input);
            aoc::DayResults::new(part1(&schematic), part2(&schematic))
        }
        None => aoc::DayResults::new(
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
        ),
    };
    input_res.print("=> Actual Input Results:");
}
