use std::cmp::Ordering;

use crate::{ParseResult, Solution, SolutionResult};
use nom::branch::alt;
use nom::{bytes::complete::tag, character::complete::u64, multi::separated_list0, IResult};

#[derive(Clone)]
enum Value {
    List(Vec<Value>),
    Num(u64),
}

impl Value {
    fn cmp(&self, other: &Value) -> Ordering {
        match (self, other) {
            (Value::Num(x), Value::Num(y)) => x.cmp(y),
            (Value::List(x), Value::List(y)) => {
                for i in 0.. {
                    if x.len() == i && y.len() > i {
                        return Ordering::Less;
                    } else if x.len() > i && y.len() == i {
                        return Ordering::Greater;
                    } else if x.len() == i && y.len() == i {
                        return Ordering::Equal;
                    } else {
                        let result = x[i].cmp(&y[i]);
                        if !result.is_eq() {
                            return result;
                        }
                    }
                }
                Ordering::Equal
            }
            (Value::Num(_), Value::List(_)) => Value::List(vec![self.clone()]).cmp(other),
            (Value::List(_), Value::Num(_)) => self.cmp(&Value::List(vec![other.clone()])),
        }
    }
}

#[derive(Default)]
pub(crate) struct Day13 {
    signals: Vec<(Value, Value)>,
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let (input, v) = u64(input)?;

    Ok((input, Value::Num(v)))
}

fn parse(input: &str) -> IResult<&str, Value> {
    let (input, _) = tag("[")(input)?;
    let (input, value) = separated_list0(tag(","), alt((parse_value, parse)))(input)?;
    let (input, _) = tag("]")(input)?;

    Ok((input, Value::List(value)))
}

impl Solution for Day13 {
    fn parse(&mut self, mut lines: impl Iterator<Item = String>) -> ParseResult {
        loop {
            let line1 = lines.next().ok_or("No more lines found")?;
            let line2 = lines.next().ok_or("No more lines found")?;

            self.signals.push((
                parse(&line1).map_err(|_| "Error in line 1")?.1,
                parse(&line2).map_err(|_| "Error in line 2")?.1,
            ));

            if lines.next().is_none() {
                break;
            }
        }

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let sum: usize = self
            .signals
            .iter()
            .enumerate()
            .filter_map(|(i, (a, b))| a.cmp(b).is_lt().then_some(i + 1))
            .sum();

        Ok(sum.to_string())
    }

    fn solve2(&self) -> SolutionResult {
        let additional1 = Value::List(vec![Value::List(vec![Value::Num(2)])]);
        let additional2 = Value::List(vec![Value::List(vec![Value::Num(6)])]);
        let mut all = self
            .signals
            .iter()
            .flat_map(|(a, b)| [a, b])
            .collect::<Vec<_>>();
        all.push(&additional1);
        all.push(&additional2);

        all.sort_by(|a, b| a.cmp(b));

        let product: usize = all
            .iter()
            .enumerate()
            .filter_map(|(i, val)| {
                (val.cmp(&additional1).is_eq() || val.cmp(&additional2).is_eq()).then_some(i + 1)
            })
            .product();

        Ok(product.to_string())
    }

    fn file_name(&self) -> &'static str {
        "day13.txt"
    }
}
