use std::error::Error;

use crate::{ParseResult, Solution, SolutionResult};
use nom::{bytes::complete::tag, character::complete::u64, IResult};

struct Instruction {
    amount: u64,
    from: u64,
    to: u64,
}

#[derive(Default)]
pub(crate) struct Day5 {
    instructions: Vec<Instruction>,
    stacks: [Vec<char>; 9],
}

fn parse(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = u64(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = u64(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = u64(input)?;

    Ok((
        input,
        Instruction {
            amount,
            from: from - 1,
            to: to - 1,
        },
    ))
}

impl Solution for Day5 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        self.stacks = [
            vec!['B', 'G', 'S', 'C'],
            vec!['T', 'M', 'W', 'H', 'J', 'N', 'V', 'G'],
            vec!['M', 'Q', 'S'],
            vec!['B', 'S', 'L', 'T', 'W', 'N', 'M'],
            vec!['J', 'Z', 'F', 'T', 'V', 'G', 'W', 'P'],
            vec!['C', 'T', 'B', 'G', 'Q', 'H', 'S'],
            vec!['T', 'J', 'P', 'B', 'W'],
            vec!['G', 'D', 'C', 'Z', 'F', 'T', 'Q', 'M'],
            vec!['N', 'S', 'H', 'B', 'P', 'F'],
        ];

        self.instructions = input_lines
            .skip(10)
            .map(|line| Ok(parse(&line).map_err(|_| "Unable to parse input")?.1))
            .collect::<Result<_, Box<dyn Error>>>()?;

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut stacks = self.stacks.clone();
        for instruction in &self.instructions {
            for _ in 0..instruction.amount {
                let c = stacks[instruction.from as usize].pop().unwrap_or('?');
                stacks[instruction.to as usize].push(c);
            }
        }
        Ok(stacks
            .iter()
            .map(|s| s.last().unwrap_or(&'?').to_owned())
            .collect())
    }

    fn solve2(&self) -> SolutionResult {
        let mut stacks = self.stacks.clone();
        for instruction in &self.instructions {
            let mut tmp = vec![];
            for _ in 0..instruction.amount {
                let c = stacks[instruction.from as usize].pop().unwrap_or('?');
                tmp.push(c);
            }
            for _ in 0..instruction.amount {
                let c = tmp.pop().unwrap_or('?');
                stacks[instruction.to as usize].push(c);
            }
        }
        Ok(stacks
            .iter()
            .map(|s| s.last().unwrap_or(&'?').to_owned())
            .collect())
    }

    fn file_name(&self) -> &'static str {
        "day5.txt"
    }
}
