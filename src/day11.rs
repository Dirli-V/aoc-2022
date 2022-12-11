use std::cmp::Reverse;

use crate::{ParseResult, Solution, SolutionResult};

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    op: fn(usize) -> usize,
    test: fn(usize) -> usize,
}

#[derive(Default)]
pub(crate) struct Day11 {
    monkeys: Vec<Monkey>,
}

impl Solution for Day11 {
    fn parse(&mut self, _: impl Iterator<Item = String>) -> ParseResult {
        self.monkeys = vec![
            Monkey {
                items: vec![78, 53, 89, 51, 52, 59, 58, 85],
                op: |old| old * 3,
                test: |val| if val % 5 == 0 { 2 } else { 7 },
            },
            Monkey {
                items: vec![64],
                op: |old| old + 7,
                test: |val| if val % 2 == 0 { 3 } else { 6 },
            },
            Monkey {
                items: vec![71, 93, 65, 82],
                op: |old| old + 5,
                test: |val| if val % 13 == 0 { 5 } else { 4 },
            },
            Monkey {
                items: vec![67, 73, 95, 75, 56, 74],
                op: |old| old + 8,
                test: |val| if val % 19 == 0 { 6 } else { 0 },
            },
            Monkey {
                items: vec![85, 91, 90],
                op: |old| old + 4,
                test: |val| if val % 11 == 0 { 3 } else { 1 },
            },
            Monkey {
                items: vec![67, 96, 69, 55, 70, 83, 62],
                op: |old| old * 2,
                test: |val| if val % 3 == 0 { 4 } else { 1 },
            },
            Monkey {
                items: vec![53, 86, 98, 70, 64],
                op: |old| old + 6,
                test: |val| if val % 7 == 0 { 7 } else { 0 },
            },
            Monkey {
                items: vec![88, 64],
                op: |old| old * old,
                test: |val| if val % 17 == 0 { 2 } else { 5 },
            },
        ];

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let mut monkeys = self.monkeys.clone();
        let mut monkey_business = vec![0; monkeys.len()];

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();
                for item in monkey.items {
                    let worry = (monkey.op)(item) / 3;
                    let target = (monkey.test)(worry);
                    monkeys[target].items.push(worry);
                }
                monkey_business[i] += monkeys[i].items.len();
                monkeys[i].items.clear();
            }
        }

        let mut all_business = monkey_business.iter().map(Reverse).collect::<Vec<_>>();
        all_business.sort();
        let score: usize = all_business.iter().take(2).map(|Reverse(x)| *x).product();

        Ok(score.to_string())
    }

    fn solve2(&self) -> SolutionResult {
        let mut monkeys = self.monkeys.clone();
        let mut monkey_business = vec![0; monkeys.len()];

        for _ in 0..10_000 {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();
                for item in monkey.items {
                    let worry = (monkey.op)(item);
                    let worry = worry % (5 * 2 * 13 * 19 * 11 * 3 * 7 * 17);
                    let target = (monkey.test)(worry);
                    monkeys[target].items.push(worry);
                }
                monkey_business[i] += monkeys[i].items.len();
                monkeys[i].items.clear();
            }
        }

        let mut all_business = monkey_business.iter().map(Reverse).collect::<Vec<_>>();
        all_business.sort();
        let score: usize = all_business.iter().take(2).map(|Reverse(x)| *x).product();

        Ok(score.to_string())
    }

    fn file_name(&self) -> &'static str {
        "day11.txt"
    }
}
