use crate::{ParseResult, Solution, SolutionResult};

#[derive(Default)]
pub(crate) struct Day1 {
    calories: Vec<usize>,
}

impl Solution for Day1 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        let mut cur_calories = 0;
        for line in input_lines {
            if line.is_empty() {
                self.calories.push(cur_calories);
                cur_calories = 0;
                continue;
            }

            cur_calories += line.parse::<usize>()?
        }

        self.calories.push(cur_calories);

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let max_calories = self.calories.iter().max().ok_or("There are no elves")?;

        Ok(format!("{}", max_calories.to_owned()))
    }

    fn solve2(&self) -> SolutionResult {
        let mut calories = self.calories.clone();
        calories.sort();
        calories.reverse();
        let total_calories = self.calories.iter().take(3).sum::<usize>();

        Ok(format!("{total_calories}"))
    }

    fn file_name(&self) -> &'static str {
        "day1.txt"
    }
}
