const YEAR: &'static str = "2023";
const DAY: &'static str = "04";

use regex::Regex;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Card {
    pub id: usize,
    pub winning: Vec<u8>,
    pub mine: Vec<u8>,
}

impl Card {
    pub fn winning_numbers(&self) -> Vec<u8> {
        let winning_set: HashSet<_> = self.winning.clone().into_iter().collect();

        self.mine
            .iter()
            .filter(|n| winning_set.contains(n))
            .cloned()
            .collect::<Vec<u8>>()
    }

    pub fn points(&self) -> u32 {
        2_u32.pow(self.winning_numbers().len() as u32 - 1)
    }
}

fn part1(cards: &Vec<Card>) -> String {
    cards.iter().map(|c| c.points()).sum::<u32>().to_string()
}

fn part2(cards: &Vec<Card>) -> String {
    let mut card_counts: HashMap<usize, usize> = cards.iter().map(|c| (c.id, 1)).collect();

    for card in cards {
        let i_won_x_cards = card.winning_numbers().len();
        let next_copies = card.id + i_won_x_cards;
        let upper_limit = if next_copies < cards.len() {
            next_copies
        } else {
            cards.len() - 1
        };
        for copy_id in card.id + 1..=upper_limit {
            card_counts.insert(
                copy_id,
                card_counts.get(&copy_id).unwrap() + card_counts.get(&card.id).unwrap(),
            );
        }
    }

    card_counts.values().sum::<usize>().to_string()
}

fn convert_nums(nums_str: &str) -> Vec<u8> {
    nums_str
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}

fn lines_to_cards(lines: &Vec<String>) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    for line in lines {
        let mut id: usize = 0;
        let id_split = line
            .split(':')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let id_re = Regex::new(r"Card\s+(\d+)").unwrap();
        id = id_re.captures(id_split.get(0).unwrap()).unwrap()[1]
            .parse::<usize>()
            .unwrap();

        let numbers_split = id_split
            .get(1)
            .unwrap()
            .split('|')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        cards.push(Card {
            id,
            winning: convert_nums(numbers_split.get(0).unwrap()),
            mine: convert_nums(numbers_split.get(1).unwrap()),
        });
    }

    cards
}

fn main() {
    let inputs = aoc::Inputs::new(YEAR, DAY);

    let sample_res = match &inputs.sample {
        Some(input) => {
            let cards = lines_to_cards(input);
            aoc::DayResults::new(part1(&cards), part2(&cards))
        }
        None => aoc::DayResults::new(
            "<SAMPLE INPUT UNAVAILABLE>".to_string(),
            "<SAMPLE INPUT UNAVAILABLE>".to_string(),
        ),
    };
    sample_res.print("=> Sample Results:");

    let input_res = match &inputs.input {
        Some(input) => {
            let cards = lines_to_cards(input);
            aoc::DayResults::new(part1(&cards), part2(&cards))
        }
        None => aoc::DayResults::new(
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
            "<ACTUAL INPUT UNAVAILABLE>".to_string(),
        ),
    };
    input_res.print("=> Actual Input Results:");
}
