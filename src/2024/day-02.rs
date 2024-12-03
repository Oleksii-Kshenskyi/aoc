const YEAR: &'static str = "2024";
const DAY: &'static str = "02";

use std::cmp::Ordering;

use aoc::Conv;

#[derive(Debug)]
pub struct Reports {
    reports: Vec<Vec<u32>>,
}

/*          incdec  range
    7 6 4 2 1 [true true]
    1 2 7 8 9 [true false]
    9 7 6 2 1 [true false]
    1 3 2 4 5 [false true]
    8 6 4 4 1 [false false]
    1 3 6 7 9 [true true]
*/

impl Reports {
    pub fn new(lines: &Vec<String>) -> Self {
        Self {
            reports: Conv::to_matrix(lines),
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
        let (inc_or_dec, within_range) = self.base_conditions();
        let pairs = self.pair_windows();
        let orderings: Vec<Vec<Ordering>> = pairs
            .iter()
            .map(|pairvec| {
                pairvec
                    .iter()
                    .map(|pair| pair[0].cmp(&pair[1]))
                    .collect::<Vec<_>>()
            })
            .collect();
        let ordpairs: Vec<Vec<&[Ordering]>> = orderings
            .iter()
            .map(|ords| ords.windows(2).collect())
            .collect();
        let problemcounts: Vec<bool> = ordpairs
            .iter()
            .map(|ordvec| {
                let sins = ordvec.iter().fold(0, |acc, pair| {
                    if (pair[0] == Ordering::Greater && pair[1] == Ordering::Greater)
                        || (pair[0] == Ordering::Less && pair[1] == Ordering::Less)
                    {
                        acc
                    } else {
                        acc + 1
                    }
                }) as u8;
                sins / 2
            })
            .map(|prc| prc < 2)
            .collect();

        dbg!(&problemcounts);
        dbg!(&inc_or_dec);
        dbg!(&within_range);
        problemcounts
            .iter()
            .zip(inc_or_dec)
            .zip(within_range)
            .map(|((prc, iod), wr)| ((wr || *prc) && iod) as u8)
            .collect::<Vec<u8>>()
    }

    fn pair_windows(&self) -> Vec<Vec<&[u32]>> {
        self.reports
            .iter()
            .map(|report| report.windows(2).collect::<Vec<&[u32]>>())
            .collect()
    }

    fn base_conditions(&self) -> (Vec<bool>, Vec<bool>) {
        let pairs = self.pair_windows();
        let inc = pairs
            .iter()
            .map(|pairvec| {
                pairvec
                    .iter()
                    .map(|pair| pair[0].cmp(&pair[1]))
                    .all(|o| o == Ordering::Greater)
            })
            .collect::<Vec<_>>();
        let dec = pairs
            .iter()
            .map(|pairvec| {
                pairvec
                    .iter()
                    .map(|pair| pair[0].cmp(&pair[1]))
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
                    .all(|pair| pair[0].abs_diff(pair[1]) > 0 && pair[0].abs_diff(pair[1]) <= 3)
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
