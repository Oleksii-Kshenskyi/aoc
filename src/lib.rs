use std::{env, fs};

fn get_file_lines(filename: &str) -> Option<Vec<String>> {
    match fs::read_to_string(format!(
        "{}/inputs/{}",
        env::current_dir().unwrap().display(),
        filename
    )) {
        Ok(s) => Some(s.lines().map(String::from).collect()),
        Err(_) => None,
    }
}

pub struct Inputs {
    pub sample: Option<Vec<String>>,
    pub input: Option<Vec<String>>,
}

impl Inputs {
    pub fn new(year: &str, day: &str) -> Self {
        let sample_filename = format!("{}-day-{}-sample.txt", year, day);
        let input_filename = format!("{}-day-{}-input.txt", year, day);
        let sample = get_file_lines(&sample_filename);
        let input = get_file_lines(&input_filename);

        Self { sample, input }
    }
}

pub struct DayResults {
    pub part1: String,
    pub part2: String,
}

impl DayResults {
    pub fn new(one: String, two: String) -> DayResults {
        Self {
            part1: one,
            part2: two,
        }
    }

    pub fn print(&self, prompt: &str) {
        println!(
            "{}\n\tPart 1: {}.\n\tPart 2: {}.",
            prompt, self.part1, self.part2
        )
    }
}

#[derive(Debug)]
pub struct CharField {
    field: Vec<Vec<char>>,
    capacity: (usize, usize),
}

impl CharField {
    pub fn from_lines(lines: &Vec<String>) -> Option<Self> {
        let ex_len = lines.get(0).unwrap().len();
        if !lines.iter().map(|l| l.len()).all(|ln| ln == ex_len) {
            return None;
        }

        let mut new_field: Vec<Vec<char>> = vec![];
        for line in lines {
            new_field.push(line.chars().collect());
        }

        Some(Self {
            field: new_field,
            capacity: (lines.len(), ex_len),
        })
    }

    pub fn num_rows(&self) -> usize {
        self.field.len()
    }

    pub fn num_cols(&self) -> usize {
        self.field.get(0).unwrap_or(&vec![]).len()
    }

    pub fn get(&self, row: usize, col: usize) -> Result<char, &'static str> {
        if row >= self.num_rows() || col >= self.num_cols() {
            return Err("Cannot get char in CharField: index out of bounds");
        }

        Ok(self.field.get(row).unwrap().get(col).unwrap().clone())
    }

    pub fn set(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if row >= self.num_rows() || col >= self.num_cols() {
            return Err("Cannot set char in CharField: index out of bounds");
        }

        Ok(())
    }
}
