use crate::{ParseResult, Solution, SolutionResult};

#[derive(Default)]
pub(crate) struct Day8 {
    heights: Vec<Vec<i8>>,
}

impl Solution for Day8 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        self.heights = input_lines
            .map(|row| {
                row.chars()
                    .map(|tree| tree.to_digit(10).unwrap_or(0) as i8)
                    .collect()
            })
            .collect();

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let row_count = self.heights.len();
        let col_count = self.heights[0].len();
        let mut visible_grid = vec![vec![0; col_count]; row_count];

        (0..row_count).for_each(|r| {
            let mut previous = -1;
            for c in 0..col_count {
                let h = self.heights[r][c];
                if h > previous {
                    previous = h;
                    visible_grid[r][c] = 1;
                }
            }
        });

        (0..row_count).for_each(|r| {
            let mut previous = -1;
            for c in (0..col_count).rev() {
                let h = self.heights[r][c];
                if h > previous {
                    previous = h;
                    visible_grid[r][c] = 1;
                }
            }
        });

        for c in 0..col_count {
            let mut previous = -1;
            (0..row_count).for_each(|r| {
                let h = self.heights[r][c];
                if h > previous {
                    previous = h;
                    visible_grid[r][c] = 1;
                }
            });
        }

        for c in 0..col_count {
            let mut previous = -1;
            for r in (0..row_count).rev() {
                let h = self.heights[r][c];
                if h > previous {
                    previous = h;
                    visible_grid[r][c] = 1;
                }
            }
        }

        let total_count: isize = visible_grid.iter().map(|r| r.iter().sum::<isize>()).sum();

        Ok(total_count.to_string())
    }

    fn solve2(&self) -> SolutionResult {
        let row_count = self.heights.len();
        let col_count = self.heights[0].len();
        let mut max_score = 0;

        for i in 0..row_count {
            for j in 0..col_count {
                let mut score = 1;
                let height = self.heights[i][j];
                let mut t = 0;
                for r in (i + 1)..row_count {
                    t += 1;
                    if height <= self.heights[r][j] {
                        break;
                    }
                }
                score *= t;
                t = 0;

                for r in (0..i).rev() {
                    t += 1;
                    if height <= self.heights[r][j] {
                        break;
                    }
                }
                score *= t;
                t = 0;

                for c in (j + 1)..col_count {
                    t += 1;
                    if height <= self.heights[i][c] {
                        break;
                    }
                }
                score *= t;
                t = 0;

                for c in (0..j).rev() {
                    t += 1;
                    if height <= self.heights[i][c] {
                        break;
                    }
                }
                score *= t;

                max_score = score.max(max_score);
            }
        }

        Ok(max_score.to_string())
    }

    fn file_name(&self) -> &'static str {
        "day8.txt"
    }
}
