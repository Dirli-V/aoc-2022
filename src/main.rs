use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod day1;
mod day2;
mod day3;
mod day4;

type SolutionResult = Result<usize, Box<dyn Error>>;
type ParseResult = Result<(), Box<dyn Error>>;

trait Solution {
    fn file_name(&self) -> &'static str;
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult;
    fn solve1(&self) -> SolutionResult;
    fn solve2(&self) -> SolutionResult;
}

fn main() {
    #[cfg(feature = "all")]
    {
        solve(day1::Day1::default());
        solve(day2::Day2::default());
        solve(day3::Day3::default());
    }
    solve(day4::Day4::default());
}

fn solve(mut s: impl Solution) {
    let file_name = s.file_name();
    let input_path = Path::new("inputs").join(file_name);
    let path = match input_path.to_str() {
        Some(p) => p,
        None => {
            println!("Unable to create path to input file: {}", file_name);
            return;
        }
    };
    let input_lines = match read_file(path) {
        Ok(it) => it,
        Err(e) => {
            println!("Unable to read file {}: {}", path, e);
            return;
        }
    };
    println!("Solving {}:", file_name);
    if let Err(e) = s.parse(input_lines) {
        println!("Error while parsing: {}", e);
        return;
    }
    match s.solve1() {
        Ok(0) => println!("The 1. solution is not yet implemented"),
        Ok(result) => println!("The 1. result is {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match s.solve2() {
        Ok(0) => println!("The 2. solution is not yet implemented"),
        Ok(result) => println!("The 2. result is {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn read_file(file_name: &str) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(|line| line.ok()))
}
