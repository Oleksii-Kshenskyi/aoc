use aoc::CharField;

const YEAR: &'static str = "2023";
const DAY: &'static str = "03";

#[derive(Debug)]
struct FieldNumber {
    pub row: usize,
    pub col_start: usize,
    pub col_end: usize,
    pub value: u16,
}

impl FieldNumber {
    fn zeroed() -> Self {
        Self {
            row: 0,
            col_start: 0,
            col_end: 0,
            value: 0,
        }
    }
}

struct Schematic {
    pub field: CharField,
    pub numbers: Option<Vec<FieldNumber>>,
}

impl Schematic {
    fn is_symbol(&self, row: usize, col: usize) -> bool {
        if let Ok(ch) = self.field.get(row, col) {
            return !ch.is_alphanumeric() && ch != '.';
        }
        false
    }

    fn extract_numbers(field: &CharField) -> Option<Vec<FieldNumber>> {
        let mut numbers: Vec<FieldNumber> = vec![];
        for currow in 0..field.num_rows() {
            let mut curcol = 0;
            let mut str_num = "".to_string();
            let mut col_start: usize = 0;
            let mut col_end: usize = 0;
            while curcol < field.num_cols() {
                if let Ok(ch) = field.get(currow, curcol) {
                    if ch.is_numeric() {
                        let left = field.left(currow, curcol);
                        let right = field.right(currow, curcol);
                        if left.is_err() || !left.unwrap().is_numeric() {
                            col_start = curcol;
                        }
                        if right.is_err() || !right.unwrap().is_numeric() {
                            col_end = curcol;
                        }
                        str_num.push(ch)
                    }

                    let left = field.left(currow, curcol);
                    if !ch.is_numeric() && left.is_ok() && left.unwrap().is_numeric() {
                        numbers.push(FieldNumber {
                            row: currow,
                            col_start,
                            col_end,
                            value: str_num.parse::<u16>().unwrap(),
                        });
                        str_num = "".to_string();
                        col_start = 0;
                        col_end = 0;
                    }
                    curcol += 1;
                }
            }
        }

        if numbers.len() > 0 {
            Some(numbers)
        } else {
            None
        }
    }

    pub fn new(input: &Vec<String>) -> Self {
        let field = CharField::from_lines(input).unwrap();
        Self {
            numbers: Self::extract_numbers(&field),
            field,
        }
    }

    pub fn adjacents(&self) -> Vec<u32> {
        let mut adj: Vec<u32> = vec![];
        //let ch: char = 'r'; ch.is_ascii
        adj
    }
}

fn part1(schematic: &Schematic) -> String {
    schematic.adjacents().iter().sum::<u32>().to_string()
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
