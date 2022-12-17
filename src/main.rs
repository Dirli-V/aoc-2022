#![feature(anonymous_lifetime_in_impl_trait)]

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

type SolutionResult = Result<String, Box<dyn Error>>;
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
        solve(day4::Day4::default());
        solve(day5::Day5::default());
        solve(day6::Day6::default());
        solve(day7::Day7::default());
        solve(day8::Day8::default());
        solve(day9::Day9::default());
        solve(day10::Day10::default());
        solve(day11::Day11::default());
        solve(day12::Day12::default());
        solve(day13::Day13::default());
        solve(day14::Day14::default());
        solve(day15::Day15::default());
    }
    solve(day16::Day16::default());
}

fn solve(mut s: impl Solution) {
    let file_name = s.file_name();
    let input_path = Path::new("inputs").join(file_name);
    let path = match input_path.to_str() {
        Some(p) => p,
        None => {
            println!("Unable to create path to input file: {file_name}");
            return;
        }
    };
    let input_lines = match read_file(path) {
        Ok(it) => it,
        Err(e) => {
            println!("Unable to read file {path}: {e}");
            return;
        }
    };
    println!("Solving {file_name}:");
    if let Err(e) = s.parse(input_lines) {
        println!("Error while parsing: {e}");
        return;
    }
    match s.solve1() {
        Ok(v) if v.is_empty() => println!("The 1. solution is not yet implemented"),
        Ok(result) => println!("The 1. result is {result}"),
        Err(e) => println!("Error: {e}"),
    }
    match s.solve2() {
        Ok(v) if v.is_empty() => println!("The 2. solution is not yet implemented"),
        Ok(result) => println!("The 2. result is {result}"),
        Err(e) => println!("Error: {e}"),
    }
    println!();
}

fn read_file(file_name: &str) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(|line| line.ok()))
}
