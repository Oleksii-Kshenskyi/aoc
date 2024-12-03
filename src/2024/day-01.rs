const YEAR: &'static str = "2024";
const DAY: &'static str = "01";

use aoc::Conv;
use std::collections::HashMap;

#[derive(Debug)]
struct Lists {
    pub numbers: Vec<Vec<u32>>,
}

impl Lists {
    pub fn new(lines: &Vec<String>) -> Self {
        Self {
            numbers: Conv::to_matrix(lines),
        }
    }

    pub fn vectors(&self) -> Vectors {
        Vectors::new(&self)
    }
}

#[derive(Debug)]
struct Vectors {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Vectors {
    pub fn new(lists: &Lists) -> Self {
        Self {
            left: lists
                .numbers
                .iter()
                .map(|pair| {
                    *pair
                        .get(0)
                        .expect("VECTORS ERROR: new(): no 0th element in pair?")
                })
                .collect(),
            right: lists
                .numbers
                .iter()
                .map(|pair| {
                    *pair
                        .get(1)
                        .expect("VECTORS ERROR: new(): no 1st element in pair?")
                })
                .collect(),
        }
    }

    pub fn sort(&mut self) -> &mut Self {
        self.left.sort();
        self.right.sort();
        self
    }

    pub fn distances(&self) -> Vec<u32> {
        self.left.iter().enumerate().map(|(i, elem)| {
            elem.abs_diff(*self.right.get(i).expect("ERROR: Vectors::distances(): a right element corresponding to the left element does not exist!"))
        }).collect()
    }

    pub fn freqs(self) -> VecFreQ {
        VecFreQ::new(self)
    }
}

#[derive(Debug)]
struct VecFreQ {
    left: Vec<u32>,
    rfreq: HashMap<u32, u32>,
}

impl VecFreQ {
    pub fn new(vecs: Vectors) -> Self {
        Self {
            left: vecs.left,
            rfreq: Self::freq_vec(&vecs.right),
        }
    }

    pub fn similarities(&self) -> Vec<u32> {
        self.left
            .iter()
            .map(|num| *num * self.rfreq.get(num).map_or(0, |f| *f))
            .collect()
    }

    fn freq_vec(data: &Vec<u32>) -> HashMap<u32, u32> {
        let mut freqs: HashMap<u32, u32> = HashMap::new();
        for elem in data {
            let value = freqs.entry(*elem).or_insert(0);
            *value += 1;
        }

        freqs
    }
}

fn part1(lists: &Vec<String>) -> String {
    Lists::new(lists)
        .vectors()
        .sort()
        .distances()
        .iter()
        .map(|ust| *ust as u64)
        .sum::<u64>()
        .to_string()
}

fn part2(lists: &Vec<String>) -> String {
    Lists::new(lists)
        .vectors()
        .freqs()
        .similarities()
        .iter()
        .map(|ust| *ust as u64)
        .sum::<u64>()
        .to_string()
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
