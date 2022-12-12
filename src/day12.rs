use std::collections::HashSet;

use crate::{ParseResult, Solution, SolutionResult};

#[derive(Default)]
pub(crate) struct Day12 {
    heights: Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
    other_starts: Vec<(usize, usize)>,
}

impl Solution for Day12 {
    fn parse(&mut self, lines: impl Iterator<Item = String>) -> ParseResult {
        self.heights = lines
            .enumerate()
            .map(|(i, line)| {
                let chars = line.chars();
                chars
                    .enumerate()
                    .map(|(j, c)| {
                        if c == 'S' {
                            self.start = (i, j);
                            1
                        } else if c == 'E' {
                            self.end = (i, j);
                            26
                        } else {
                            if c == 'a' {
                                self.other_starts.push((i, j));
                            }
                            c as usize - 'a' as usize + 1
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut next = vec![(self.start, 0)];

        while let Some(((i, j), steps)) = next.pop() {
            if visited.contains(&(i, j)) {
                continue;
            }

            if self.end == (i, j) {
                return Ok(steps.to_string());
            }

            let h = self.heights[i][j];

            if i > 0 && self.heights[i - 1][j] - 1 <= h {
                next.push(((i - 1, j), steps + 1));
            }
            if i < self.heights.len() - 1 && self.heights[i + 1][j] - 1 <= h {
                next.push(((i + 1, j), steps + 1));
            }
            if j > 0 && self.heights[i][j - 1] - 1 <= h {
                next.push(((i, j - 1), steps + 1));
            }
            if j < self.heights[0].len() - 1 && self.heights[i][j + 1] - 1 <= h {
                next.push(((i, j + 1), steps + 1));
            }

            visited.insert((i, j));
            next.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        }

        Err("No solution found".into())
    }

    fn solve2(&self) -> SolutionResult {
        let mut best = usize::MAX;

        for start in &self.other_starts {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let mut next = vec![(*start, 0)];

            while let Some(((i, j), steps)) = next.pop() {
                if visited.contains(&(i, j)) {
                    continue;
                }

                if self.end == (i, j) {
                    best = best.min(steps);
                }

                let h = self.heights[i][j];

                if i > 0 && self.heights[i - 1][j] - 1 <= h {
                    next.push(((i - 1, j), steps + 1));
                }
                if i < self.heights.len() - 1 && self.heights[i + 1][j] - 1 <= h {
                    next.push(((i + 1, j), steps + 1));
                }
                if j > 0 && self.heights[i][j - 1] - 1 <= h {
                    next.push(((i, j - 1), steps + 1));
                }
                if j < self.heights[0].len() - 1 && self.heights[i][j + 1] - 1 <= h {
                    next.push(((i, j + 1), steps + 1));
                }

                visited.insert((i, j));
                next.sort_by(|a, b| a.1.cmp(&b.1).reverse());
            }
        }

        Ok(best.to_string())
    }

    fn file_name(&self) -> &'static str {
        "day12.txt"
    }
}
