use crate::{ParseResult, Solution, SolutionResult};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Default)]
pub(crate) struct Day2 {
    guide: Vec<(Shape, Shape)>,
}

impl Solution for Day2 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        self.guide = input_lines
            .map(|line| {
                line.split_once(' ')
                    .map(|(l, r)| (l.to_string(), r.to_string()))
                    .ok_or("Unable to convert input")
            })
            .map(|game| {
                let game = game?;
                Ok((
                    match game.0.as_str() {
                        "A" => Shape::Rock,
                        "B" => Shape::Paper,
                        "C" => Shape::Scissors,
                        _ => return Err("Invalid guide"),
                    },
                    match game.1.as_str() {
                        "X" => Shape::Rock,
                        "Y" => Shape::Paper,
                        "Z" => Shape::Scissors,
                        _ => return Err("Invalid guide"),
                    },
                ))
            })
            .collect::<Result<_, _>>()?;

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let score = self
            .guide
            .iter()
            .map(|game| match game {
                (Shape::Rock, Shape::Rock) => 1 + 3,
                (Shape::Rock, Shape::Paper) => 2 + 6,
                (Shape::Rock, Shape::Scissors) => 3 + 0,
                (Shape::Paper, Shape::Rock) => 1 + 0,
                (Shape::Paper, Shape::Paper) => 2 + 3,
                (Shape::Paper, Shape::Scissors) => 3 + 6,
                (Shape::Scissors, Shape::Rock) => 1 + 6,
                (Shape::Scissors, Shape::Paper) => 2 + 0,
                (Shape::Scissors, Shape::Scissors) => 3 + 3,
            })
            .sum();

        Ok(score)
    }

    fn solve2(&self) -> SolutionResult {
        let score = self
            .guide
            .iter()
            .map(|game| match game {
                (Shape::Rock, Shape::Rock) => 3 + 0,
                (Shape::Rock, Shape::Paper) => 1 + 3,
                (Shape::Rock, Shape::Scissors) => 2 + 6,
                (Shape::Paper, Shape::Rock) => 1 + 0,
                (Shape::Paper, Shape::Paper) => 2 + 3,
                (Shape::Paper, Shape::Scissors) => 3 + 6,
                (Shape::Scissors, Shape::Rock) => 2 + 0,
                (Shape::Scissors, Shape::Paper) => 3 + 3,
                (Shape::Scissors, Shape::Scissors) => 1 + 6,
            })
            .sum();

        Ok(score)
    }

    fn file_name(&self) -> &'static str {
        "day2.txt"
    }
}
