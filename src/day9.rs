use std::{collections::HashSet, error::Error};

use crate::{ParseResult, Solution, SolutionResult};

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Default)]
pub(crate) struct Day9 {
    moves: Vec<(Direction, usize)>,
}

impl Solution for Day9 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        self.moves = input_lines
            .map(|line| {
                let (d, a) = line.split_once(' ').ok_or("No valid moved")?;
                let d = match d {
                    "R" => Direction::Right,
                    "L" => Direction::Left,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    _ => return Err("No valid direction".into()),
                };
                let a = a.parse()?;
                Ok((d, a))
            })
            .collect::<Result<_, Box<dyn Error>>>()?;

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        let mut hi = 0;
        let mut hj = 0;
        let mut ti = 0;
        let mut tj = 0;

        for m in &self.moves {
            for _ in 0..m.1 {
                match m.0 {
                    Direction::Up => hj -= 1,
                    Direction::Down => hj += 1,
                    Direction::Left => hi -= 1,
                    Direction::Right => hi += 1,
                }

                let idiff = hi - ti;
                let jdiff = hj - tj;

                if idiff == 2 {
                    ti = hi - 1;
                    tj = hj;
                }
                if idiff == -2 {
                    ti = hi + 1;
                    tj = hj;
                }
                if jdiff == 2 {
                    ti = hi;
                    tj = hj - 1;
                }
                if jdiff == -2 {
                    ti = hi;
                    tj = hj + 1;
                }

                visited.insert((ti, tj));
            }
        }

        let count = visited.len();
        Ok(count.to_string())
    }

    fn solve2(&self) -> SolutionResult {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        let mut pos = vec![(0, 0); 10];

        for m in &self.moves {
            for _ in 0..m.1 {
                match m.0 {
                    Direction::Up => pos[0].1 -= 1,
                    Direction::Down => pos[0].1 += 1,
                    Direction::Left => pos[0].0 -= 1,
                    Direction::Right => pos[0].0 += 1,
                }

                for i in 1..pos.len() {
                    let hi: isize = pos[i - 1].0;
                    let hj: isize = pos[i - 1].1;
                    let mut ti = pos[i].0;
                    let mut tj = pos[i].1;
                    let idiff: isize = hi - ti;
                    let jdiff: isize = hj - tj;

                    if idiff.abs() == 2 && jdiff.abs() == 2 {
                        ti += idiff / 2;
                        tj += jdiff / 2;
                    } else {
                        if idiff == 2 {
                            ti = hi - 1;
                            tj = hj;
                        }
                        if idiff == -2 {
                            ti = hi + 1;
                            tj = hj;
                        }
                        if jdiff == 2 {
                            ti = hi;
                            tj = hj - 1;
                        }
                        if jdiff == -2 {
                            ti = hi;
                            tj = hj + 1;
                        }
                    }

                    pos[i].0 = ti;
                    pos[i].1 = tj;
                }

                visited.insert((pos[9].0, pos[9].1));
            }
        }

        let count = visited.len();
        Ok(count.to_string())
    }

    fn file_name(&self) -> &'static str {
        "day9.txt"
    }
}
