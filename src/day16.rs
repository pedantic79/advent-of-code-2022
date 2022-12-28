use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, multi::separated_list1, sequence::tuple, IResult};
use std::cmp::Reverse;

use crate::common::nom::nom_u32;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    rate: usize,
    connections: Vec<usize>,
}

fn parse_line(s: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    let (s, (_, name, _, rate, _, connections)) = tuple((
        tag("Valve "),
        nom::bytes::complete::take(2usize),
        tag(" has flow rate="),
        nom_u32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), nom::bytes::complete::take(2usize)),
    ))(s)?;

    Ok((s, (name, rate, connections)))
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Vec<Object> {
    let valves = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .map(|(k, v1, v2)| (k, (v1 as usize, v2)))
        .collect::<HashMap<_, _>>();
    let mut rev_lookup = Vec::with_capacity(valves.len());
    let mut lookup = HashMap::with_capacity(valves.len());
    lookup.insert("AA", 0);
    rev_lookup.push("AA");
    let mut idx = 1;

    for k in valves.keys() {
        if *k != "AA" {
            lookup.insert(k, idx);
            rev_lookup.push(k);
            idx += 1;
        }
    }

    let mut res = Vec::with_capacity(valves.len());
    for k in rev_lookup {
        let connections = valves[k].1.iter().map(|x| lookup[x] as usize).collect();
        res.push(Object {
            rate: valves[k].0,
            connections,
        })
    }

    res
}

#[inline]
fn encode(n: usize) -> usize {
    1usize << n
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    open: usize,
    current_room: usize,
    flow_per: usize,
    total: usize,
}

impl State {
    fn next(&self, object: &[Object]) -> Vec<Self> {
        let mut res = vec![];
        let total = self.total + self.flow_per;

        // Open Valve:
        if object[self.current_room].rate > 0 && !self.open & encode(self.current_room) > 0 {
            let mut open = self.open;
            open |= encode(self.current_room);
            res.push(Self {
                open,
                total,
                flow_per: self.flow_per + object[self.current_room].rate,
                current_room: self.current_room,
            });
        }

        // Move
        for next_valve in &object[self.current_room].connections {
            res.push(Self {
                current_room: *next_valve,
                total,
                open: self.open,
                flow_per: self.flow_per,
            });
        }
        res
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ElephantState {
    open: usize,
    current_room: usize,
    elephant_room: usize,
    flow_per: usize,
    total: usize,
}

impl ElephantState {
    fn next(&self, object: &[Object]) -> Vec<Self> {
        let mut vec_you = vec![];
        let total = self.total + self.flow_per;

        if object[self.current_room].rate > 0 && !self.open & (encode(self.current_room)) > 0 {
            let mut open = self.open;
            open |= encode(self.current_room);
            vec_you.push((
                Self {
                    open,
                    current_room: self.current_room,
                    elephant_room: self.elephant_room,
                    flow_per: self.flow_per + object[self.current_room].rate,
                    total,
                },
                true,
            ));
        }

        for next_valve in &object[self.current_room].connections {
            vec_you.push((
                Self {
                    open: self.open,
                    current_room: *next_valve,
                    elephant_room: self.elephant_room,
                    flow_per: self.flow_per,
                    total,
                },
                false,
            ));
        }

        let mut vec_elephant = vec![];
        if object[self.elephant_room].rate > 0 && !self.open & (encode(self.elephant_room)) > 0 {
            let mut open = self.open;
            open |= encode(self.elephant_room);
            vec_elephant.push((
                Self {
                    open,
                    current_room: self.current_room,
                    elephant_room: self.elephant_room,
                    flow_per: self.flow_per + object[self.elephant_room].rate,
                    total,
                },
                true,
            ));
        }

        for next_valve in &object[self.elephant_room].connections {
            vec_elephant.push((
                Self {
                    open: self.open,
                    current_room: self.current_room,
                    elephant_room: *next_valve,
                    flow_per: self.flow_per,
                    total,
                },
                false,
            ));
        }

        let mut res = vec![];
        for ((you, you_opened), (elephant, elephant_opened)) in vec_you
            .into_iter()
            .cartesian_product(vec_elephant.into_iter())
        {
            let current_room = you.current_room;
            let elephant_room = elephant.elephant_room;
            if you_opened && elephant_opened {
                if you.current_room != elephant.elephant_room {
                    let mut open = you.open;
                    open |= encode(self.elephant_room);

                    res.push(ElephantState {
                        open,
                        current_room,
                        elephant_room,
                        flow_per: you.flow_per + object[elephant.elephant_room].rate,
                        total,
                    });
                }
            } else {
                let open = if you.open.count_ones() > elephant.open.count_ones() {
                    you.open
                } else {
                    elephant.open
                };

                res.push(ElephantState {
                    open,
                    current_room,
                    elephant_room,
                    flow_per: you.flow_per.max(elephant.flow_per),
                    total,
                });
            }
        }
        res
    }
}

#[aoc(day16, part1)]
pub fn part1(inputs: &[Object]) -> usize {
    let mut state = vec![State {
        open: Default::default(),
        current_room: 0,
        flow_per: 0,
        total: 0,
    }];
    for _ in 0..30 {
        state = state
            .into_iter()
            .flat_map(|state| state.next(inputs).into_iter())
            .collect();
        state.sort_unstable_by_key(|state| Reverse(state.total));
        state.truncate(425);
    }

    state
        .into_iter()
        .max_by_key(|state| state.total)
        .unwrap()
        .total
}

#[aoc(day16, part2)]
pub fn part2(inputs: &[Object]) -> usize {
    let mut state = vec![ElephantState {
        open: Default::default(),
        current_room: 0,
        elephant_room: 0,
        flow_per: 0,
        total: 0,
    }];
    for _ in 0..26 {
        state = state
            .into_iter()
            .flat_map(|state| state.next(inputs).into_iter())
            .collect();
        state.sort_unstable_by_key(|state| Reverse(state.total));
        state.truncate(3075);
    }

    state
        .into_iter()
        .max_by_key(|state| state.total)
        .unwrap()
        .total
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 1651);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 1707);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day16.txt");
        const ANSWERS: (usize, usize) = (1376, 1933);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
