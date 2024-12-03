const YEAR: &'static str = "2024";
const DAY: &'static str = "03";

use std::u32;

use aoc::Conv;
use regex::Regex;

#[derive(Debug)]
pub enum CommandType {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(Debug)]
struct Capsule {
    pub cmd: String,
    pub op1: String,
    pub op2: String,
}
impl Default for Capsule {
    fn default() -> Self {
        Self {
            cmd: "".to_owned(),
            op1: "".to_owned(),
            op2: "".to_owned(),
        }
    }
}
impl Capsule {
    pub fn to_command(&self) -> CommandType {
        match self.cmd.as_str() {
            "do" => CommandType::Do,
            "dont" => CommandType::Dont,
            "mul" => CommandType::Mul(
                self.op1.parse::<u32>().unwrap_or(u32::MAX),
                self.op2.parse::<u32>().unwrap_or(u32::MAX),
            ),
            _ => unreachable!("Capsule::to_command(): unexpected command type!"),
        }
    }
}

#[derive(Debug)]
struct Memory {
    data: String,
    commands: Vec<CommandType>,
    filtered_muls: Vec<CommandType>,
    enabled: bool,
}

impl Memory {
    pub fn new(lines: &Vec<String>) -> Self {
        Self {
            data: Conv::to_string(lines),
            commands: vec![],
            filtered_muls: vec![],
            enabled: true,
        }
    }

    pub fn muls(&self) -> Vec<u32> {
        let mul_pattern = r"mul\((\d+),(\d+)\)";
        let rgx = Regex::new(mul_pattern).expect("ERROR: Memory::muls(): incorrect regex?");

        let mul_ops: Vec<(u32, u32)> = rgx
            .captures_iter(&self.data)
            .filter_map(|cap| Some((cap.get(1).unwrap(), cap.get(2).unwrap())))
            .filter_map(|(op1m, op2m)| {
                Some((
                    op1m.as_str().parse::<u32>().unwrap_or(u32::MAX),
                    op2m.as_str().parse::<u32>().unwrap_or(u32::MAX),
                ))
            })
            .collect::<Vec<(u32, u32)>>();

        mul_ops.iter().map(|(op1, op2)| *op1 * *op2).collect()
    }

    pub fn commands(mut self) -> Self {
        let mul_pattern = r"(mul\((\d+),(\d+)\)|do\(\)|don\'t\(\))";
        let rgx = Regex::new(mul_pattern).expect("ERROR: Memory::commands(): incorrect regex?");
        let capsules: Vec<Capsule> = rgx
            .captures_iter(&self.data)
            .map(|c| {
                let mut tempcap = Capsule::default();
                if let Some(cmd) = c.get(1) {
                    if cmd.as_str().starts_with("mul") {
                        tempcap.cmd = "mul".to_owned();
                    } else if cmd.as_str().starts_with("don't") {
                        tempcap.cmd = "dont".to_owned()
                    } else if cmd.as_str().starts_with("do") {
                        tempcap.cmd = "do".to_owned()
                    } else {
                        unreachable!(
                            "Memory::commands(): command is none of the expected options!"
                        );
                    }
                }
                tempcap.op1 = if let Some(op1) = c.get(2) {
                    op1.as_str().to_owned()
                } else {
                    "".to_owned()
                };
                tempcap.op2 = if let Some(op2) = c.get(3) {
                    op2.as_str().to_owned()
                } else {
                    "".to_owned()
                };

                tempcap
            })
            .collect();

        self.commands = capsules.iter().map(|cp| cp.to_command()).collect();
        self
    }

    pub fn filtered(mut self) -> Self {
        self.filtered_muls = vec![];
        for cmd in &self.commands {
            match cmd {
                CommandType::Do => self.enabled = true,
                CommandType::Dont => self.enabled = false,
                CommandType::Mul(op1, op2) => {
                    if self.enabled {
                        self.filtered_muls.push(CommandType::Mul(*op1, *op2));
                    }
                }
            }
        }

        self
    }

    pub fn run(&self) -> Vec<u32> {
        self.filtered_muls
            .iter()
            .filter(|ct| matches!(*ct, CommandType::Mul(_, _)))
            .filter_map(|mul| Self::run_mul(&mul))
            .collect()
    }

    fn run_mul(cmd: &CommandType) -> Option<u32> {
        match *cmd {
            CommandType::Mul(op1, op2) => Some(op1 * op2),
            _ => None,
        }
    }
}

fn part1(lines: &Vec<String>) -> String {
    Memory::new(lines)
        .muls()
        .into_iter()
        .sum::<u32>()
        .to_string()
}

fn part2(lines: &Vec<String>) -> String {
    Memory::new(lines)
        .commands()
        .filtered()
        .run()
        .iter()
        .map(|e| *e as u64)
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
