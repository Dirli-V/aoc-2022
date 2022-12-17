use std::collections::{HashMap, HashSet};

use crate::{ParseResult, Solution, SolutionResult};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i64},
    multi::separated_list1,
    IResult,
};

#[derive(Clone)]
struct Reading {
    id: String,
    flow: i64,
    targets: Vec<String>,
}

#[derive(Default)]
pub(crate) struct Day16 {
    readings: Vec<Reading>,
}

fn parse_id(input: &str) -> IResult<&str, String> {
    let (input, a) = anychar(input)?;
    let (input, b) = anychar(input)?;

    Ok((input, format!("{a}{b}")))
}

fn parse(input: &str) -> IResult<&str, Reading> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, id) = parse_id(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow) = i64(input)?;
    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(input)?;
    let (input, targets) = separated_list1(tag(", "), parse_id)(input)?;

    Ok((input, Reading { id, flow, targets }))
}

impl Solution for Day16 {
    fn parse(&mut self, lines: impl Iterator<Item = String>) -> ParseResult {
        self.readings = lines
            .map(|l| parse(&l).map_err(|_| "Unable to parse line").map(|v| v.1))
            .collect::<Result<_, _>>()?;

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let valves = self
            .readings
            .iter()
            .map(|reading| (reading.id.clone(), reading.clone()))
            .collect::<HashMap<_, _>>();

        let mut known_moves = HashMap::new();
        let total = make_move1(&valves["AA"], &HashSet::new(), 0, &valves, &mut known_moves);

        Ok(total.to_string())
    }

    fn solve2(&self) -> SolutionResult {
        let mut valves = self
            .readings
            .iter()
            .map(|reading| (reading.id.clone(), reading.clone()))
            .collect::<HashMap<_, _>>();

        let r = valves.clone();
        for v in valves.values_mut() {
            v.targets.sort_by(|a, b| r[a].flow.cmp(&r[b].flow));
        }

        let mut best_best = 0;
        let mut known_moves = HashMap::new();
        let total = make_move2(
            &valves["AA"],
            &valves["AA"],
            &HashSet::new(),
            0,
            &valves,
            &mut known_moves,
            &mut best_best,
            0,
        );

        Ok(total.to_string())
    }

    fn file_name(&self) -> &'static str {
        "day16.txt"
    }
}

fn make_move1(
    cur_valve: &Reading,
    open_valves: &HashSet<String>,
    past_minutes: u8,
    valves: &HashMap<String, Reading>,
    known_moves: &mut HashMap<(String, u8, String), i64>,
) -> i64 {
    const MAX_MINUTE: u8 = 30;
    if past_minutes == MAX_MINUTE {
        return 0;
    }
    let key = (
        cur_valve.id.to_owned(),
        past_minutes,
        open_valves.iter().join(","),
    );
    if let Some(&v) = known_moves.get(&key) {
        return v;
    }
    let mut best = if cur_valve.flow > 0 && !open_valves.contains(&cur_valve.id) {
        let mut open_valves = open_valves.clone();
        open_valves.insert(cur_valve.id.clone());
        cur_valve.flow * (MAX_MINUTE - (past_minutes + 1)) as i64
            + make_move1(
                cur_valve,
                &open_valves,
                past_minutes + 1,
                valves,
                known_moves,
            )
    } else {
        0
    };

    for id in &cur_valve.targets {
        best = best.max(make_move1(
            &valves[id],
            open_valves,
            past_minutes + 1,
            valves,
            known_moves,
        ));
    }

    known_moves.insert(key, best);
    best
}

fn potential_gain(
    valves: &HashMap<String, Reading>,
    open_valves: &HashSet<String>,
    remaining_minutes: u8,
) -> i64 {
    let mut total = 0;
    if remaining_minutes == 0 {
        return total;
    }
    let mut mult = remaining_minutes - 1;
    if mult == 0 {
        return total;
    }
    let mut closed_valves = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .filter(|(id, _)| !open_valves.contains(*id))
        .map(|(_, v)| v.flow)
        .collect::<Vec<_>>();
    closed_valves.sort_by(|a, b| b.cmp(a));
    let mut it = closed_valves.iter();
    #[allow(clippy::while_let_loop)]
    loop {
        if let Some(v) = it.next() {
            total += v * mult as i64;
        } else {
            break;
        }
        if let Some(v) = it.next() {
            total += v * mult as i64;
        } else {
            break;
        }
        if mult == 1 {
            break;
        }
        mult -= 2;
        if mult == 0 {
            break;
        }
    }
    total
}

#[allow(clippy::too_many_arguments)]
fn make_move2(
    cur_valve: &Reading,
    e_cur_valve: &Reading,
    open_valves: &HashSet<String>,
    past_minutes: u8,
    valves: &HashMap<String, Reading>,
    known_moves: &mut HashMap<(String, String, u8, String), i64>,
    best_best: &mut i64,
    path_gains: i64,
) -> i64 {
    const MAX_MINUTE: u8 = 26;
    if past_minutes == MAX_MINUTE {
        return 0;
    }
    if potential_gain(valves, open_valves, MAX_MINUTE - past_minutes) + path_gains <= *best_best {
        return 0;
    }
    let key = (
        cur_valve.id.to_owned(),
        e_cur_valve.id.to_owned(),
        past_minutes,
        open_valves.iter().join(","),
    );
    if let Some(&v) = known_moves.get(&key) {
        return v;
    }
    let key2 = (
        e_cur_valve.id.to_owned(),
        cur_valve.id.to_owned(),
        past_minutes,
        open_valves.iter().join(","),
    );
    if let Some(&v) = known_moves.get(&key2) {
        return v;
    }
    let open_me = cur_valve.flow > 0 && !open_valves.contains(&cur_valve.id);
    let open_e = e_cur_valve.flow > 0 && !open_valves.contains(&e_cur_valve.id);
    let mut best = 0;
    if open_me && open_e && cur_valve.id != e_cur_valve.id {
        let mut open_valves = open_valves.clone();
        open_valves.insert(cur_valve.id.clone());
        open_valves.insert(e_cur_valve.id.clone());
        let added_gains =
            (cur_valve.flow + e_cur_valve.flow) * (MAX_MINUTE - (past_minutes + 1)) as i64;
        let b = added_gains
            + make_move2(
                cur_valve,
                e_cur_valve,
                &open_valves,
                past_minutes + 1,
                valves,
                known_moves,
                best_best,
                path_gains + added_gains,
            );
        if b + path_gains > *best_best {
            *best_best = b + path_gains;
        }
        best = best.max(b);
    } else {
        if open_me {
            let mut open_valves = open_valves.clone();
            open_valves.insert(cur_valve.id.clone());
            for id in &e_cur_valve.targets {
                let added_gains = cur_valve.flow * (MAX_MINUTE - (past_minutes + 1)) as i64;
                let b = added_gains
                    + make_move2(
                        cur_valve,
                        &valves[id],
                        &open_valves,
                        past_minutes + 1,
                        valves,
                        known_moves,
                        best_best,
                        path_gains + added_gains,
                    );
                if b + path_gains > *best_best {
                    *best_best = b + path_gains;
                }
                best = best.max(b);
            }
        }
        if open_e {
            let mut open_valves = open_valves.clone();
            open_valves.insert(e_cur_valve.id.clone());
            for id in &cur_valve.targets {
                let added_gains = e_cur_valve.flow * (MAX_MINUTE - (past_minutes + 1)) as i64;
                let b = added_gains
                    + make_move2(
                        &valves[id],
                        e_cur_valve,
                        &open_valves,
                        past_minutes + 1,
                        valves,
                        known_moves,
                        best_best,
                        path_gains + added_gains,
                    );
                if b + path_gains > *best_best {
                    *best_best = b + path_gains;
                }
                best = best.max(b);
            }
        }
    }

    for id in &cur_valve.targets {
        for e_id in &e_cur_valve.targets {
            let b = make_move2(
                &valves[id],
                &valves[e_id],
                open_valves,
                past_minutes + 1,
                valves,
                known_moves,
                best_best,
                path_gains,
            );
            if b + path_gains > *best_best {
                *best_best = b + path_gains;
            }
            best = best.max(b);
        }
    }

    known_moves.insert(key, best);
    best
}
