use std::error::Error;
use std::ops::RangeInclusive;

use crate::{ParseResult, Solution, SolutionResult};
use nom::{bytes::complete::tag, character::complete::u64, IResult};

#[derive(Default)]
pub(crate) struct Day4 {
    assignments: Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>,
}

fn parse(input: &str) -> IResult<&str, (RangeInclusive<u64>, RangeInclusive<u64>)> {
    let (input, from1) = u64(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, to1) = u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, from2) = u64(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, to2) = u64(input)?;

    Ok((input, (from1..=to1, from2..=to2)))
}

impl Solution for Day4 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        self.assignments = input_lines
            .map(|line| Ok(parse(&line).map_err(|_| "Unable to parse input")?.1))
            .collect::<Result<_, Box<dyn Error>>>()?;

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let count = self
            .assignments
            .iter()
            .filter(|(p1, p2)| {
                p1.contains(p2.start()) && p1.contains(p2.end())
                    || p2.contains(p1.start()) && p2.contains(p1.end())
            })
            .count();
        Ok(count)
    }

    fn solve2(&self) -> SolutionResult {
        let count = self
            .assignments
            .iter()
            .filter(|(p1, p2)| {
                p1.contains(p2.start())
                    || p1.contains(p2.end())
                    || p2.contains(p1.start())
                    || p2.contains(p1.end())
            })
            .count();
        Ok(count)
    }

    fn file_name(&self) -> &'static str {
        "day4.txt"
    }
}
