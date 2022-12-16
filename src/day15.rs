use std::collections::HashSet;

use crate::{ParseResult, Solution, SolutionResult};
use nom::{bytes::complete::tag, character::complete::i64, IResult};

struct Pos {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

#[derive(Default)]
pub(crate) struct Day15 {
    pos: Vec<Pos>,
}

fn parse(input: &str) -> IResult<&str, Pos> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sx) = i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sy) = i64(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, bx) = i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, by) = i64(input)?;

    Ok((input, Pos { sx, sy, bx, by }))
}

impl Solution for Day15 {
    fn parse(&mut self, lines: impl Iterator<Item = String>) -> ParseResult {
        self.pos = lines
            .map(|l| parse(&l).map_err(|_| "Unable to parse line").map(|v| v.1))
            .collect::<Result<_, _>>()?;

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        const IMPORTANT_Y: i64 = 2_000_000;
        let mut filled_spots = HashSet::new();
        let mut beacon_spots = HashSet::new();

        for pos in &self.pos {
            if pos.by == IMPORTANT_Y {
                beacon_spots.insert(pos.bx);
            }
            let distance_beacon = pos.bx.abs_diff(pos.sx) + pos.by.abs_diff(pos.sy);
            let distance_important_y = pos.sy.abs_diff(IMPORTANT_Y);
            if distance_beacon < distance_important_y {
                continue;
            }
            let diff = distance_beacon - distance_important_y;
            filled_spots.insert(pos.sx);
            for i in 1..=diff {
                filled_spots.insert(pos.sx - i as i64);
                filled_spots.insert(pos.sx + i as i64);
            }
        }

        Ok((filled_spots.len() - beacon_spots.len()).to_string())
    }

    fn solve2(&self) -> SolutionResult {
        const DIM: i64 = 4_000_000;
        let mut cur_x = 0;
        let mut cur_y = 0;
        'outer: loop {
            if cur_y > DIM {
                break;
            }

            for pos in &self.pos {
                let distance_beacon = pos.bx.abs_diff(pos.sx) + pos.by.abs_diff(pos.sy);
                let distance_important = pos.sx.abs_diff(cur_x) + pos.sy.abs_diff(cur_y);
                if distance_beacon < distance_important {
                    continue;
                }
                let diff = distance_beacon - distance_important;
                cur_x += diff as i64 + 1;
                if cur_x > DIM {
                    cur_x = 0;
                    cur_y += 1;
                }
                continue 'outer;
            }

            return Ok((cur_x * DIM + cur_y).to_string());
        }

        Ok("None found".to_string())
    }

    fn file_name(&self) -> &'static str {
        "day15.txt"
    }
}
