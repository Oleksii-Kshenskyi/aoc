use aoc::CharField;

use std::{collections::HashSet, hash::Hash};

const YEAR: &'static str = "2023";
const DAY: &'static str = "03";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct FieldNumber {
    row: usize,
    col_start: usize,
    col_end: usize,
    value: u16,
}

impl FieldNumber {
    fn new(row: usize, col_start: usize, col_end: usize, value: u16) -> Self {
        Self {
            row,
            col_start,
            col_end,
            value,
        }
    }

    pub fn is_adjacent_to(&self, fch: &FieldChar) -> bool {
        (self.row >= fch.row - 1 && self.row <= fch.row + 1)
            && ((self.col_start >= fch.col - 1 && self.col_start <= fch.col + 1)
                || self.col_end >= fch.col - 1 && self.col_end <= fch.col + 1)
    }
}

struct FieldChar {
    row: usize,
    col: usize,
    ch: char,
}

impl FieldChar {
    pub fn new(ch: char, row: usize, col: usize) -> Self {
        FieldChar { ch, row, col }
    }
}

struct Schematic {
    pub field: CharField,
    pub numbers: Option<Vec<FieldNumber>>,
}

impl Schematic {
    fn is_symbol(&self, row: usize, col: usize) -> Option<FieldChar> {
        if let Ok(ch) = self.field.get(row, col) {
            if !ch.is_alphanumeric() && ch != '.' {
                return Some(FieldChar::new(ch, row, col));
            }
        }
        None
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
                    let right = field.right(currow, curcol);
                    if (!ch.is_numeric() && left.is_ok() && left.unwrap().is_numeric())
                        || (ch.is_numeric() && right.is_err())
                    {
                        numbers.push(FieldNumber::new(
                            currow,
                            col_start,
                            col_end,
                            str_num.parse::<u16>().unwrap(),
                        ));
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

    fn gear_adjacents(&self) -> Vec<Vec<FieldNumber>> {
        let mut gear_adjs: Vec<Vec<FieldNumber>> = vec![];
        let sym_adjs = self.adjacents();
        let gear_chars = self.find_gears();

        for gear_ch in gear_chars {
            let relevant_adjs = sym_adjs
                .iter()
                .filter(|adj| adj.row >= gear_ch.row - 1 && adj.row <= gear_ch.row + 1)
                .collect::<Vec<&FieldNumber>>();
            let cur_adjs = relevant_adjs
                .into_iter()
                .filter(|adj| adj.is_adjacent_to(&gear_ch))
                .collect::<Vec<&FieldNumber>>();

            let mut unique_checker: HashSet<FieldNumber> = HashSet::new();
            gear_adjs.push(
                cur_adjs
                    .into_iter()
                    .cloned()
                    .filter(|adj| unique_checker.insert(adj.clone()))
                    .collect(),
            );
        }

        gear_adjs
    }

    pub fn new(input: &Vec<String>) -> Self {
        let field = CharField::from_lines(input).unwrap();
        Self {
            numbers: Self::extract_numbers(&field),
            field,
        }
    }

    pub fn adjacents(&self) -> Vec<FieldNumber> {
        let mut adj: Vec<FieldNumber> = vec![];
        for num in self.numbers.as_ref().unwrap() {
            let mut is_adjacent = false;
            if self.is_symbol(num.row - 1, num.col_start - 1).is_some() || // ul
            self.is_symbol(num.row + 1, num.col_start - 1).is_some() || // dl
            self.is_symbol(num.row - 1, num.col_end + 1).is_some() || // ur
            self.is_symbol(num.row + 1, num.col_end + 1).is_some()
            // dr
            {
                is_adjacent = true;
            }
            for curcol in num.col_start..=num.col_end {
                if self.is_symbol(num.row - 1, curcol).is_some()
                    || self.is_symbol(num.row + 1, curcol).is_some()
                {
                    is_adjacent = true;
                }
            }
            if self.is_symbol(num.row, num.col_start - 1).is_some()
                || self.is_symbol(num.row, num.col_end + 1).is_some()
            {
                is_adjacent = true;
            }

            if is_adjacent {
                adj.push(num.clone());
            }
        }
        adj
    }

    pub fn find_gears(&self) -> Vec<FieldChar> {
        let mut gears: Vec<FieldChar> = vec![];
        for row in 0..self.field.num_rows() {
            for col in 0..self.field.num_cols() {
                if let Some(ch) = self.is_symbol(row, col) {
                    if ch.ch == '*' {
                        gears.push(ch);
                    }
                }
            }
        }

        gears
    }

    pub fn find_gear_ratios(&self) -> Vec<u32> {
        self.gear_adjacents()
            .iter()
            .filter(|ga| ga.len() == 2)
            .map(|ga| ga.get(0).unwrap().value as u32 * ga.get(1).unwrap().value as u32)
            .collect::<Vec<u32>>()
    }
}

fn part1(schematic: &Schematic) -> String {
    schematic
        .adjacents()
        .iter()
        .map(|fien| fien.value as u32)
        .sum::<u32>()
        .to_string()
}

fn part2(schematic: &Schematic) -> String {
    schematic.find_gear_ratios().iter().sum::<u32>().to_string()
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
