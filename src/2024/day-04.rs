const YEAR: &'static str = "2024";
const DAY: &'static str = "04";

use aoc::{CharField, CharFieldDirection};

#[derive(Debug)]
struct WordSearch {
    pub field: CharField,
    pub count: u64,
}
impl WordSearch {
    pub fn new(lines: &Vec<String>) -> Self {
        Self {
            field: CharField::from_lines(lines)
                .expect("ERROR: CharField::new(): couldn't create a CharField from these lines!"),
            count: 0,
        }
    }

    pub fn horizontal_forward(mut self, word: &str) -> Self {
        for row_index in 0..self.field.num_rows() {
            for col_index in 0..self.field.num_cols() - word.len() + 1 {
                if self
                    .field
                    .find_word((row_index, col_index), word, CharFieldDirection::Forward)
                {
                    self.count += 1;
                }
            }
        }

        self
    }

    pub fn horizontal_backwards(mut self, word: &str) -> Self {
        for row_index in 0..self.field.num_rows() {
            for col_index in CharField::rev(self.field.num_cols() - 1, word.len() - 1) {
                if self
                    .field
                    .find_word((row_index, col_index), word, CharFieldDirection::Backwards)
                {
                    self.count += 1;
                }
            }
        }

        self
    }

    pub fn vertical_down(mut self, word: &str) -> Self {
        for col_index in 0..self.field.num_cols() {
            for row_index in 0..self.field.num_rows() - word.len() + 1 {
                if self
                    .field
                    .find_word((row_index, col_index), word, CharFieldDirection::Down)
                {
                    self.count += 1;
                }
            }
        }

        self
    }

    pub fn vertical_up(mut self, word: &str) -> Self {
        for col_index in 0..self.field.num_cols() {
            for row_index in CharField::rev(self.field.num_rows() - 1, word.len() - 1) {
                if self
                    .field
                    .find_word((row_index, col_index), word, CharFieldDirection::Up)
                {
                    self.count += 1;
                }
            }
        }

        self
    }

    pub fn diagonal_down_right(mut self, word: &str) -> Self {
        for row_index in 0..self.field.num_rows() - word.len() + 1 {
            for col_index in 0..self.field.num_cols() - word.len() + 1 {
                if self.field.find_word(
                    (row_index, col_index),
                    word,
                    CharFieldDirection::DiagDownRight,
                ) {
                    self.count += 1;
                }
            }
        }

        self
    }

    pub fn diagonal_up_left(mut self, word: &str) -> Self {
        for row_index in CharField::rev(self.field.num_rows() - 1, word.len() - 1) {
            for col_index in CharField::rev(self.field.num_cols() - 1, word.len() - 1) {
                if self.field.find_word(
                    (row_index, col_index),
                    word,
                    CharFieldDirection::DiagUpLeft,
                ) {
                    self.count += 1;
                }
            }
        }

        self
    }

    pub fn diagonal_down_left(mut self, word: &str) -> Self {
        for row_index in 0..self.field.num_rows() - word.len() + 1 {
            for col_index in CharField::rev(self.field.num_cols() - 1, word.len() - 1) {
                if self.field.find_word(
                    (row_index, col_index),
                    word,
                    CharFieldDirection::DiagDownLeft,
                ) {
                    self.count += 1;
                }
            }
        }

        self
    }

    pub fn diagonal_up_right(mut self, word: &str) -> Self {
        for row_index in CharField::rev(self.field.num_rows() - 1, word.len() - 1) {
            for col_index in 0..self.field.num_cols() - word.len() + 1 {
                if self.field.find_word(
                    (row_index, col_index),
                    word,
                    CharFieldDirection::DiagUpRight,
                ) {
                    self.count += 1;
                }
            }
        }

        self
    }

    pub fn x_mases(mut self) -> Self {
        for row_index in 1..self.field.num_rows() - 1 {
            for col_index in 1..self.field.num_cols() - 1 {
                if self.field.find_word(
                    (row_index - 1, col_index - 1),
                    "MAS",
                    CharFieldDirection::DiagDownRight,
                ) {
                    if self.field.find_word(
                        (row_index + 1, col_index - 1),
                        "MAS",
                        CharFieldDirection::DiagUpRight,
                    ) {
                        self.count += 1;
                    }
                }
                if self.field.find_word(
                    (row_index - 1, col_index - 1),
                    "MAS",
                    CharFieldDirection::DiagDownRight,
                ) {
                    if self.field.find_word(
                        (row_index - 1, col_index + 1),
                        "MAS",
                        CharFieldDirection::DiagDownLeft,
                    ) {
                        self.count += 1;
                    }
                }
                if self.field.find_word(
                    (row_index + 1, col_index + 1),
                    "MAS",
                    CharFieldDirection::DiagUpLeft,
                ) {
                    if self.field.find_word(
                        (row_index + 1, col_index - 1),
                        "MAS",
                        CharFieldDirection::DiagUpRight,
                    ) {
                        self.count += 1;
                    }
                }
                if self.field.find_word(
                    (row_index - 1, col_index + 1),
                    "MAS",
                    CharFieldDirection::DiagDownLeft,
                ) {
                    if self.field.find_word(
                        (row_index + 1, col_index + 1),
                        "MAS",
                        CharFieldDirection::DiagUpLeft,
                    ) {
                        self.count += 1;
                    }
                }
            }
        }

        self
    }

    pub fn counted(&self) -> u64 {
        self.count
    }
}

fn part1(lines: &Vec<String>) -> String {
    WordSearch::new(lines)
        .horizontal_forward("XMAS")
        .horizontal_backwards("XMAS")
        .vertical_down("XMAS")
        .vertical_up("XMAS")
        .diagonal_down_right("XMAS")
        .diagonal_up_left("XMAS")
        .diagonal_down_left("XMAS")
        .diagonal_up_right("XMAS")
        .counted()
        .to_string()
}

fn part2(lines: &Vec<String>) -> String {
    WordSearch::new(lines).x_mases().counted().to_string()
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
