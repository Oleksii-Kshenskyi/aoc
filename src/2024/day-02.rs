const YEAR: &'static str = "2024";
const DAY: &'static str = "02";

use std::cmp::Ordering;

use aoc::Conv;

#[derive(Debug, Clone)]
pub struct Level {
    value: u32,
}
impl Level {
    pub fn new(value: u32) -> Self {
        Level { value }
    }
}

#[derive(Debug)]
pub struct Reports {
    reports: Vec<Vec<Level>>,
}

impl Reports {
    pub fn new(lines: &Vec<String>) -> Self {
        Self {
            reports: Self::pack(Conv::to_matrix(lines)),
        }
    }

    pub fn check_safety(&self) -> Vec<u8> {
        let (inc_or_dec, within_range) = self.base_conditions();

        inc_or_dec
            .iter()
            .zip(within_range)
            .map(|(id, wr)| (*id && wr) as u8)
            .collect::<Vec<_>>()
    }

    pub fn safety_dampened(&self) -> Vec<u8> {
        self.reports
            .iter()
            .enumerate()
            .map(|(_, report)| {
                if Self::is_safe(report) {
                    return 1;
                }
    
                for i in 0..report.len() {
                    let mut modified = report.clone();
                    modified.remove(i);
    
                    if Self::is_safe(&modified) {
                        return 1;
                    }
                }
    
                0
            })
            .collect()
    }
    
    fn is_safe(report: &Vec<Level>) -> bool {
        let is_increasing = report
            .windows(2)
            .all(|pair| pair[0].value < pair[1].value);
        let is_decreasing = report
            .windows(2)
            .all(|pair| pair[0].value > pair[1].value);
        let within_range = report
            .windows(2)
            .all(|pair| pair[0].value.abs_diff(pair[1].value) <= 3);
        let inc_or_dec = is_increasing || is_decreasing;
    
        inc_or_dec && within_range
    }

    fn pack(matrix: Vec<Vec<u32>>) -> Vec<Vec<Level>> {
        matrix
            .iter()
            .map(|row| row.iter().map(|val| Level::new(*val)).collect())
            .collect::<Vec<_>>()
    }

    fn slices(&self, size: usize) -> Vec<Vec<&[Level]>> {
        self.reports
            .iter()
            .map(|report| report.windows(size).collect())
            .collect::<Vec<Vec<&[Level]>>>()
    }

    fn base_conditions(&self) -> (Vec<bool>, Vec<bool>) {
        let pairs = self.slices(2);
        let inc = pairs
            .iter()
            .map(|pairvec| {
                pairvec
                    .iter()
                    .map(|pair| pair[0].value.cmp(&pair[1].value))
                    .all(|o| o == Ordering::Greater)
            })
            .collect::<Vec<_>>();
        let dec = pairs
            .iter()
            .map(|pairvec| {
                pairvec
                    .iter()
                    .map(|pair| pair[0].value.cmp(&pair[1].value))
                    .all(|o| o == Ordering::Less)
            })
            .collect::<Vec<_>>();
        let inc_or_dec = inc
            .iter()
            .zip(dec)
            .map(|(i, d)| *i || d)
            .collect::<Vec<_>>();
        let within_range = pairs
            .iter()
            .map(|pairvec| {
                pairvec
                    .iter()
                    .all(|pair| pair[0].value.abs_diff(pair[1].value) <= 3)
            })
            .collect::<Vec<_>>();

        (inc_or_dec, within_range)
    }
}

fn part1(lists: &Vec<String>) -> String {
    Reports::new(lists)
        .check_safety()
        .iter()
        .map(|ue| *ue as u64)
        .sum::<u64>()
        .to_string()
}

fn part2(lists: &Vec<String>) -> String {
    Reports::new(lists)
        .safety_dampened()
        .iter()
        .map(|ue| *ue as u64)
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
