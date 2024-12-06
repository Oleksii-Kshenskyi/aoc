const YEAR: &'static str = "2024";
const DAY: &'static str = "05";

use std::collections::HashMap;

use aoc::Conv;

#[derive(Debug)]
struct Pages {
    rules: HashMap<u32, Vec<u32>>,
    pages: Vec<Vec<u32>>,
    validated_pages: Vec<Vec<u32>>,
    invalidated_pages: Vec<Vec<u32>>,
}

impl Pages {
    pub fn new(lines: &Vec<String>) -> Self {
        Self {
            rules: Self::collect_rules(lines),
            pages: Self::collect_pages(lines),
            validated_pages: vec![],
            invalidated_pages: vec![],
        }
    }

    pub fn validate_pages(mut self) -> Self {
        self.validated_pages = self
            .pages
            .iter()
            .filter_map(|manual| self.validate_manual(manual, &self.rules))
            .collect();

        self
    }

    pub fn invalidate_pages(mut self) -> Self {
        self.invalidated_pages = self
            .pages
            .iter()
            .filter_map(|manual| {
                if self.validate_manual(manual, &self.rules).is_none() {
                    Some(manual.clone())
                } else {
                    None
                }
            })
            .collect();

        self
    }

    pub fn reorder_invalid(mut self) -> Self {
        let mut invalid_clone = self.invalidated_pages.clone();
        for manual in &mut invalid_clone {
            self.reorder_manual(manual);
        }

        self.invalidated_pages = invalid_clone;
        self
    }

    pub fn middles(&self) -> Vec<u32> {
        self.validated_pages
            .iter()
            .map(|manual| *manual.get((manual.len() - 1) / 2).unwrap())
            .collect::<Vec<_>>()
    }

    pub fn invalid_middles(&self) -> Vec<u32> {
        self.invalidated_pages
            .iter()
            .map(|manual| *manual.get((manual.len() - 1) / 2).unwrap())
            .collect::<Vec<_>>()
    }

    fn reorder_manual(&self, manual: &mut Vec<u32>) {
        let mut empty_vec: Vec<u32> = vec![];
        let mut rules_clone = self.rules.clone();
        'validate_loop: while self.validate_manual(manual, &rules_clone).is_none() {
            for (index, page) in manual.iter().enumerate() {
                let my_rules = rules_clone.get_mut(page).unwrap_or(&mut empty_vec);
                let after_me = &manual[index + 1..manual.len()];
                for (after_index, after_num) in after_me.iter().enumerate() {
                    if my_rules.contains(after_num) {
                        manual.swap(index, index + 1 + after_index);
                        continue 'validate_loop;
                    }
                }
            }
        }
    }

    fn validate_manual(
        &self,
        manual: &Vec<u32>,
        rules: &HashMap<u32, Vec<u32>>,
    ) -> Option<Vec<u32>> {
        let empty_vec: Vec<u32> = vec![];
        for (index, page) in manual.iter().enumerate() {
            let my_rules = rules.get(page).unwrap_or(&empty_vec);
            let after_me = &manual[index + 1..manual.len()];
            for after_num in after_me {
                if my_rules.contains(after_num) {
                    return None;
                }
            }
        }

        Some(manual.clone())
    }

    fn rule_map(rule_vec: &Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
        let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::new();
        rule_vec.iter().for_each(|(before, after)| {
            let rules = rule_map.entry(*after).or_insert(vec![]);
            rules.push(*before);
        });

        rule_map
    }

    fn collect_rules(lines: &Vec<String>) -> HashMap<u32, Vec<u32>> {
        Self::rule_map(
            &lines
                .iter()
                .filter_map(|s| {
                    Conv::to_u32s_sep(s, "|").ok().map(|vec| {
                        match vec.as_slice() {
            [a, b] => (*a, *b),
            _ => panic!("Pages::collect_rules(): the vector does NOT consist of two elements..."),
        }
                    })
                })
                .collect(),
        )
    }

    fn collect_pages(lines: &Vec<String>) -> Vec<Vec<u32>> {
        lines
            .iter()
            .filter_map(|s| Conv::to_u32s_sep(s, ",").ok())
            .collect()
    }
}

fn part1(lines: &Vec<String>) -> String {
    Pages::new(lines)
        .validate_pages()
        .middles()
        .iter()
        .map(|m| *m as u64)
        .sum::<u64>()
        .to_string()
}

fn part2(lines: &Vec<String>) -> String {
    Pages::new(lines)
        .invalidate_pages()
        .reorder_invalid()
        .invalid_middles()
        .iter()
        .map(|m| *m as u64)
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
