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
