use crate::{ParseResult, Solution, SolutionResult};
use nom::multi::separated_list1;
use nom::{bytes::complete::tag, character::complete::u64, IResult};

#[derive(Default)]
pub(crate) struct Day14 {
    lines: Vec<Vec<(u64, u64)>>,
}

fn parse_point(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, v1) = u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, v2) = u64(input)?;

    Ok((input, (v1, v2)))
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, line) = separated_list1(tag(" -> "), parse_point)(input)?;

    Ok((input, line))
}

impl Solution for Day14 {
    fn parse(&mut self, lines: impl Iterator<Item = String>) -> ParseResult {
        self.lines = lines
            .map(|l| parse(&l).map_err(|_| "Unable to parse line").map(|v| v.1))
            .collect::<Result<_, _>>()?;

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut grid = vec![vec![false; 1000]; 1000];

        for line in &self.lines {
            for points in line.windows(2) {
                let &[(p1x, p1y), (p2x, p2y), ..] = points else {
                    return Err("Windows does not work".into());
                };

                for x in p1x.min(p2x)..=p2x.max(p1x) {
                    grid[x as usize][p1y as usize] = true;
                }
                for y in p1y.min(p2y)..=p2y.max(p1y) {
                    grid[p1x as usize][y as usize] = true;
                }
            }
        }

        for i in 0.. {
            let mut x = 500;
            let mut y = 0;

            loop {
                if y == grid.len() - 1 {
                    return Ok(i.to_string());
                }
                if !grid[x][y + 1] {
                    y += 1;
                } else if !grid[x - 1][y + 1] {
                    x -= 1;
                    y += 1;
                } else if !grid[x + 1][y + 1] {
                    x += 1;
                    y += 1;
                } else {
                    break;
                }
            }

            grid[x][y] = true;
        }

        Ok(String::new())
    }

    fn solve2(&self) -> SolutionResult {
        let mut grid = vec![vec![false; 1000]; 1000];

        let mut max_y = 0;
        for line in &self.lines {
            for points in line.windows(2) {
                let &[(p1x, p1y), (p2x, p2y), ..] = points else {
                    return Err("Windows does not work".into());
                };

                for x in p1x.min(p2x)..=p2x.max(p1x) {
                    grid[x as usize][p1y as usize] = true;
                }
                for y in p1y.min(p2y)..=p2y.max(p1y) {
                    grid[p1x as usize][y as usize] = true;
                }

                max_y = max_y.max(p1y).max(p2y);
            }
        }

        max_y += 2;
        for line in &mut grid {
            line[max_y as usize] = true;
        }

        for i in 0.. {
            let mut x = 500;
            let mut y = 0;

            if grid[x][y] {
                return Ok(i.to_string());
            }

            loop {
                if !grid[x][y + 1] {
                    y += 1;
                } else if !grid[x - 1][y + 1] {
                    x -= 1;
                    y += 1;
                } else if !grid[x + 1][y + 1] {
                    x += 1;
                    y += 1;
                } else {
                    break;
                }
            }

            grid[x][y] = true;
        }

        Ok(String::new())
    }

    fn file_name(&self) -> &'static str {
        "day14.txt"
    }
}
