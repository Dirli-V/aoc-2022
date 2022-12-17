use std::collections::HashMap;

use crate::{ParseResult, Solution, SolutionResult};

#[derive(Default)]
pub(crate) struct Day17 {
    pushes: Vec<bool>,
}

impl Solution for Day17 {
    fn parse(&mut self, mut lines: impl Iterator<Item = String>) -> ParseResult {
        self.pushes = lines.next().unwrap().chars().map(|c| c == '>').collect();

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut state = Vec::new();

        let mut pushes = self.pushes.iter().cycle();
        let mut blocks = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            vec![(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        ]
        .into_iter()
        .cycle();

        for _ in 0..2022 {
            let b = blocks.next().unwrap().to_owned();
            let top = find_top_index(&state);
            let mut offset = (2, top + 3);
            for _ in state.len()..(top + 9) as usize {
                state.push(vec![false; 7]);
            }
            loop {
                let p = pushes.next().unwrap().to_owned();
                if p || offset.0 > 0 {
                    let pot_offset = if p {
                        (offset.0 + 1, offset.1)
                    } else {
                        (offset.0 - 1, offset.1)
                    };
                    let mut blocked = false;
                    for s in &b {
                        if pot_offset.0 + s.0 >= 7 {
                            blocked = true;
                            break;
                        }
                        if state[(pot_offset.1 + s.1) as usize][pot_offset.0 + s.0] {
                            blocked = true;
                            break;
                        }
                    }
                    if !blocked {
                        offset = pot_offset;
                    }
                }
                let mut blocked = false;
                if offset.1 == 0 {
                    blocked = true;
                } else {
                    let pot_offset = (offset.0, offset.1 - 1);
                    for s in &b {
                        if state[(pot_offset.1 + s.1) as usize][pot_offset.0 + s.0] {
                            blocked = true;
                            break;
                        }
                    }
                    if !blocked {
                        offset = pot_offset;
                    }
                }
                if blocked {
                    for s in &b {
                        state[(offset.1 + s.1) as usize][offset.0 + s.0] = true;
                    }
                    break;
                }
            }
        }

        Ok(find_top_index(&state).to_string())
    }

    fn solve2(&self) -> SolutionResult {
        const TARGET: usize = 1000000000000;
        let mut state = Vec::new();

        let mut pushes = self.pushes.iter().enumerate().cycle();
        let mut blocks = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            vec![(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        ]
        .into_iter()
        .enumerate()
        .cycle();

        let mut knowns = HashMap::new();
        let mut floor_offset = 0;

        let mut i1 = 0;
        let mut i2 = 0;
        let mut nr = 0;
        loop {
            let f = find_floor(&state);
            if f > 0 {
                floor_offset += f;
                state.drain(0..f as usize);
            }
            let s = (state.clone(), i1, i2);
            #[allow(clippy::map_entry)]
            if knowns.contains_key(&s) {
                let (original_nr, original_floor_offset) = knowns[&s];
                let move_diff = nr - original_nr;
                let floor_offset_diff = floor_offset - original_floor_offset;
                let repeat = (TARGET - nr) / move_diff;
                floor_offset += repeat as i64 * floor_offset_diff;
                nr += repeat * move_diff;
                knowns.clear();
            } else {
                knowns.insert(s, (nr, floor_offset));
            }

            let (x1, b) = blocks.next().unwrap().to_owned();
            i1 = x1;
            let top = find_top_index(&state);
            let mut offset = (2, top + 3);
            for _ in state.len()..(top + 9) as usize {
                state.push(vec![false; 7]);
            }
            loop {
                let (x2, &p) = pushes.next().unwrap().to_owned();
                i2 = x2;
                if p || offset.0 > 0 {
                    let pot_offset = if p {
                        (offset.0 + 1, offset.1)
                    } else {
                        (offset.0 - 1, offset.1)
                    };
                    let mut blocked = false;
                    for s in &b {
                        if pot_offset.0 + s.0 >= 7 {
                            blocked = true;
                            break;
                        }
                        if state[(pot_offset.1 + s.1) as usize][pot_offset.0 + s.0] {
                            blocked = true;
                            break;
                        }
                    }
                    if !blocked {
                        offset = pot_offset;
                    }
                }
                let mut blocked = false;
                if offset.1 == 0 {
                    blocked = true;
                } else {
                    let pot_offset = (offset.0, offset.1 - 1);
                    for s in &b {
                        if state[(pot_offset.1 + s.1) as usize][pot_offset.0 + s.0] {
                            blocked = true;
                            break;
                        }
                    }
                    if !blocked {
                        offset = pot_offset;
                    }
                }
                if blocked {
                    for s in &b {
                        state[(offset.1 + s.1) as usize][offset.0 + s.0] = true;
                    }
                    break;
                }
            }

            nr += 1;
            if nr == TARGET {
                break;
            }
        }

        Ok((find_top_index(&state) + floor_offset).to_string())
    }

    fn file_name(&self) -> &'static str {
        "day17.txt"
    }
}

fn find_floor(state: &Vec<Vec<bool>>) -> i64 {
    for i in (0..state.len()).rev() {
        if state[i].iter().all(|v| *v) {
            return i as i64 + 1;
        }
    }
    0
}

fn find_top_index(state: &Vec<Vec<bool>>) -> i64 {
    for i in (0..state.len()).rev() {
        for j in 0..state[i].len() {
            if state[i][j] {
                return i as i64 + 1;
            }
        }
    }
    0
}
