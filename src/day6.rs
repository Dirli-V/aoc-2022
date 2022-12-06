use std::collections::HashSet;

use crate::{ParseResult, Solution, SolutionResult};

#[derive(Default)]
pub(crate) struct Day6 {
    signal: String,
}

impl Solution for Day6 {
    fn parse(&mut self, mut input_lines: impl Iterator<Item = String>) -> ParseResult {
        self.signal = input_lines.next().ok_or("Empty input")?;

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let chars = self.signal.chars().collect::<Vec<_>>();

        let (i, f) = chars
            .windows(4)
            .enumerate()
            .find(|(_, chars)| {
                let mut set = HashSet::new();
                for c in chars.iter() {
                    set.insert(*c);
                }
                set.len() == 4
            })
            .ok_or("No unique sequence found")?;

        Ok(format!("{}", i + 4))
    }

    fn solve2(&self) -> SolutionResult {
        let chars = self.signal.chars().collect::<Vec<_>>();

        let (i, f) = chars
            .windows(14)
            .enumerate()
            .find(|(_, chars)| {
                let mut set = HashSet::new();
                for c in chars.iter() {
                    set.insert(*c);
                }
                set.len() == 14
            })
            .ok_or("No unique sequence found")?;

        Ok(format!("{}", i + 14))
    }

    fn file_name(&self) -> &'static str {
        "day6.txt"
    }
}
