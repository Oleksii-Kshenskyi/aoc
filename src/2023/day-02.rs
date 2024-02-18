use std::collections::HashMap;

use regex::Regex;

const YEAR: &'static str = "2023";
const DAY: &'static str = "02";

#[derive(Debug)]
struct CubeSet {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl CubeSet {
    pub fn zeroed() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    pub id: u16,
    pub sets: Vec<CubeSet>,
    pub capacity: CubeSet,
}

impl Game {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            sets: vec![],
            capacity: CubeSet {
                red: 12,
                green: 13,
                blue: 14,
            },
        }
    }

    pub fn is_possible(&self) -> bool {
        for set in &self.sets {
            if set.red > self.capacity.red
                || set.green > self.capacity.green
                || set.blue > self.capacity.blue
            {
                return false;
            }
        }
        true
    }
}

fn input_to_games(input: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];
    for line in input {
        let ided = line
            .split(':')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let id_re = Regex::new(r"^Game\s+(\d+)$").unwrap();
        let id_caps = id_re.captures(&ided[0]).unwrap();
        let mut game = Game::new(id_caps[1].parse::<u16>().unwrap());
        let sets = ided[1].trim().split(';');

        for set in sets {
            let set_re = Regex::new(r"(\d+)\s+(red|green|blue)").unwrap();
            let mut colors: HashMap<String, u16> = HashMap::new();
            for cap in set_re.captures_iter(set) {
                let count = cap[1].to_string().parse::<u16>().unwrap();
                colors.insert(cap[2].to_string(), count);
            }
            let mut cube_set = CubeSet::zeroed();
            if colors.contains_key("red") {
                cube_set.red = *colors.get("red").unwrap();
            }
            if colors.contains_key("green") {
                cube_set.green = *colors.get("green").unwrap();
            }
            if colors.contains_key("blue") {
                cube_set.blue = *colors.get("blue").unwrap();
            }
            game.sets.push(cube_set);
        }
        games.push(game);
    }

    games
}

fn part1(input: &Vec<String>) -> String {
    input_to_games(input)
        .iter()
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .sum::<u16>()
        .to_string()
}

fn part2(_input: &Vec<String>) -> String {
    "2".to_string()
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
