use std::collections::{HashMap, HashSet};

use crate::{ParseResult, Solution, SolutionResult};

#[derive(Default)]
pub(crate) struct Day7 {
    lines: Vec<String>,
}

impl Solution for Day7 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        self.lines = input_lines.collect();

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut sizes: HashMap<String, usize> = HashMap::new();
        let mut known_files: HashSet<String> = HashSet::new();
        let mut current_path: Vec<String> = Vec::new();
        for cmd in &self.lines {
            let parts = cmd.split_whitespace().collect::<Vec<_>>();
            let first = parts.first().ok_or("Unexpected empty line")?;
            if *first == "$" {
                let second = parts.get(1).ok_or("No command found")?;
                match *second {
                    "cd" => {
                        let third = parts.get(2).ok_or("cd target not available")?;
                        if *third == "/" {
                            current_path.clear();
                        } else if *third == ".." {
                            current_path.pop();
                        } else {
                            current_path.push(third.to_string());
                        }
                        sizes.entry(current_path.join("/")).or_default();
                    }
                    "ls" => {}
                    _ => return Err("Unknown command".into()),
                }
            } else if *first != "dir" {
                let size: usize = first.parse()?;
                let second = parts.get(1).ok_or("No filename found")?;
                let mut total_path = current_path.clone();
                total_path.push(second.to_string());
                let file_name = total_path.join("/");
                if !known_files.contains(&file_name) {
                    known_files.insert(file_name);
                    total_path.pop();
                    while !total_path.is_empty() {
                        let cur = sizes.entry(total_path.join("/")).or_default();
                        *cur += size;
                        total_path.pop();
                    }
                    let cur = sizes.entry(total_path.join("/")).or_default();
                    *cur += size;
                }
            }
        }

        let sum: usize = sizes.values().filter(|size| **size <= 100_000).sum();

        Ok(sum.to_string())
    }

    fn solve2(&self) -> SolutionResult {
        let mut sizes: HashMap<String, usize> = HashMap::new();
        let mut known_files: HashSet<String> = HashSet::new();
        let mut current_path: Vec<String> = Vec::new();
        for cmd in &self.lines {
            let parts = cmd.split_whitespace().collect::<Vec<_>>();
            let first = parts.first().ok_or("Unexpected empty line")?;
            if *first == "$" {
                let second = parts.get(1).ok_or("No command found")?;
                match *second {
                    "cd" => {
                        let third = parts.get(2).ok_or("cd target not available")?;
                        if *third == "/" {
                            current_path.clear();
                        } else if *third == ".." {
                            current_path.pop();
                        } else {
                            current_path.push(third.to_string());
                        }
                        sizes.entry(current_path.join("/")).or_default();
                    }
                    "ls" => {}
                    _ => return Ok("Unknown command".to_string()),
                }
            } else if *first != "dir" {
                let size: usize = first.parse()?;
                let second = parts.get(1).ok_or("No filename found")?;
                let mut total_path = current_path.clone();
                total_path.push(second.to_string());
                let file_name = total_path.join("/");
                if !known_files.contains(&file_name) {
                    known_files.insert(file_name);
                    total_path.pop();
                    while !total_path.is_empty() {
                        let cur = sizes.entry(total_path.join("/")).or_default();
                        *cur += size;
                        total_path.pop();
                    }
                    let cur = sizes.entry(total_path.join("/")).or_default();
                    *cur += size;
                }
            }
        }

        let total_size = sizes.get("").ok_or("No root dir found")?;
        let free_space = 70_000_000 - total_size;
        let needed_space = 30_000_000 - free_space;
        let mut potential_dirs = sizes
            .values()
            .filter(|size| **size >= needed_space)
            .collect::<Vec<_>>();
        potential_dirs.sort();
        let best_option = potential_dirs.first().ok_or("No dirs available")?;

        Ok(best_option.to_string())
    }

    fn file_name(&self) -> &'static str {
        "day7.txt"
    }
}
