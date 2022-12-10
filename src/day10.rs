use crate::{ParseResult, Solution, SolutionResult};

#[derive(Default)]
pub(crate) struct Day10 {
    instructions: Vec<Option<isize>>,
}

impl Solution for Day10 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        self.instructions = input_lines
            .map(|line| line.split_once(' ').map(|(_, i)| i.parse().unwrap_or(0)))
            .collect();

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut pc = 0;
        let mut x = 1;
        let mut sum = 0;
        let mut instruction = None;
        for cycle in 1.. {
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => sum += cycle * x,
                _ => {}
            }

            match instruction {
                Some(x_diff) => {
                    x += x_diff;
                    instruction = None;
                }
                None => {
                    match self.instructions.get(pc) {
                        Some(new_instruction) => instruction = *new_instruction,
                        None => break,
                    }
                    pc += 1;
                }
            }
        }

        Ok(sum.to_string())
    }

    fn solve2(&self) -> SolutionResult {
        let mut pc = 0;
        let mut x: isize = 1;
        let mut instruction = None;
        let mut offset = 0;
        for cycle in 1.. {
            let pos = cycle - offset - 1;
            if x.abs_diff(pos) <= 1 {
                print!("#");
            } else {
                print!(".");
            }

            match instruction {
                Some(x_diff) => {
                    x += x_diff;
                    instruction = None;
                }
                None => {
                    match self.instructions.get(pc) {
                        Some(new_instruction) => instruction = *new_instruction,
                        None => break,
                    }
                    pc += 1;
                }
            }

            if cycle % 40 == 0 {
                offset = cycle;
                println!();
            }
        }

        Ok("above.".to_string())
    }

    fn file_name(&self) -> &'static str {
        "day10.txt"
    }
}
