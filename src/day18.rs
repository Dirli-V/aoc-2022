use std::{
    collections::{HashMap, HashSet},
    error::Error,
    vec,
};

use crate::{ParseResult, Solution, SolutionResult};

use nom::{bytes::complete::tag, character::complete::u64, IResult};

#[derive(Default)]
pub(crate) struct Day18 {
    cubes: Vec<(usize, usize, usize)>,
}

fn parse(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, x) = u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = u64(input)?;

    Ok((input, (x as usize + 1, y as usize + 1, z as usize + 1)))
}

impl Solution for Day18 {
    fn parse(&mut self, lines: impl Iterator<Item = String>) -> ParseResult {
        self.cubes = lines
            .map(|line| -> Result<(usize, usize, usize), Box<dyn Error>> {
                Ok(parse(&line).map_err(|_| "Unable to parse input")?.1)
            })
            .collect::<Result<_, _>>()?;

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut state = vec![vec![vec![false; 30]; 30]; 30];
        let mut total_sides = self.cubes.len() * 6;

        for c in &self.cubes {
            state[c.0][c.1][c.2] = true;
            for p in gen_next(c) {
                if state[p.0][p.1][p.2] {
                    total_sides -= 2;
                }
            }
        }

        Ok(total_sides.to_string())
    }

    fn solve2(&self) -> SolutionResult {
        let mut state = vec![vec![vec![false; 30]; 30]; 30];
        let mut total_sides = self.cubes.len() * 6;

        for c in &self.cubes {
            state[c.0][c.1][c.2] = true;
            for p in gen_next(c) {
                if state[p.0][p.1][p.2] {
                    total_sides -= 2;
                }
            }
        }

        let mut known = HashMap::new();
        for c in &self.cubes {
            for p in gen_next(c) {
                if !state[p.0][p.1][p.2] && !reaches_outside(&state, &mut known, (p.0, p.1, p.2)) {
                    total_sides -= 1;
                }
            }
        }

        Ok(total_sides.to_string())
    }

    fn file_name(&self) -> &'static str {
        "day18.txt"
    }
}

fn gen_next(p: &(usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    let mut v = Vec::new();
    if p.2 < 29 {
        v.push((p.0, p.1, p.2 + 1));
    }
    if p.2 > 0 {
        v.push((p.0, p.1, p.2 - 1));
    }
    if p.1 < 29 {
        v.push((p.0, p.1 + 1, p.2));
    }
    if p.1 > 0 {
        v.push((p.0, p.1 - 1, p.2));
    }
    if p.0 < 29 {
        v.push((p.0 + 1, p.1, p.2));
    }
    if p.0 > 0 {
        v.push((p.0 - 1, p.1, p.2));
    }
    v
}

fn reaches_outside(
    state: &[Vec<Vec<bool>>],
    known: &mut HashMap<(usize, usize, usize), bool>,
    p: (usize, usize, usize),
) -> bool {
    if let Some(&b) = known.get(&p) {
        return b;
    }

    let mut checked = HashSet::new();
    let mut next = vec![p];
    let mut free = false;
    'outer: while let Some(c) = next.pop() {
        for p in gen_next(&c) {
            if checked.contains(&p) {
                continue;
            }
            if (p.0 == 0 || p.0 == 29) && (p.1 == 0 || p.1 == 29) && (p.2 == 0 || p.2 == 29) {
                free = true;
                break 'outer;
            }
            if !state[p.0][p.1][p.2] {
                next.push(p);
            }
            checked.insert(p);
        }
    }
    for p in checked {
        known.insert(p, free);
    }
    free
}
