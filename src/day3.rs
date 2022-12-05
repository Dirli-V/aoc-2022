use crate::{ParseResult, Solution, SolutionResult};
use itertools::Itertools;

struct Rucksack {
    first: Vec<char>,
    second: Vec<char>,
}

#[derive(Default)]
pub(crate) struct Day3 {
    rucksacks: Vec<Rucksack>,
}

impl Solution for Day3 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        self.rucksacks = input_lines
            .map(|line| {
                let (l, r) = line.split_at(line.len() / 2);
                (l.to_owned(), r.to_owned())
            })
            .map(|(l, r)| Rucksack {
                first: l.chars().collect(),
                second: r.chars().collect(),
            })
            .collect();

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut total_prio = 0;
        'outer: for rucksack in &self.rucksacks {
            for item in &rucksack.first {
                if rucksack.second.contains(item) {
                    total_prio += calc_prio(*item);
                    continue 'outer;
                }
            }
        }
        Ok(format!("{total_prio}"))
    }

    fn solve2(&self) -> SolutionResult {
        let mut total_prio = 0;

        'outer: for (a, b, c) in self.rucksacks.iter().tuples() {
            for item in a.first.iter().chain(a.second.iter()) {
                let found_in_b = b.first.contains(item) || b.second.contains(item);
                let found_in_c = c.first.contains(item) || c.second.contains(item);

                if found_in_b && found_in_c {
                    total_prio += calc_prio(*item);
                    continue 'outer;
                }
            }
        }

        Ok(format!("{total_prio}"))
    }

    fn file_name(&self) -> &'static str {
        "day3.txt"
    }
}

fn calc_prio(item: char) -> usize {
    if item.is_lowercase() {
        item as usize - 'a' as usize + 1
    } else {
        item as usize - 'A' as usize + 27
    }
}
