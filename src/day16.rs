use ahash::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    rate: usize,
    connections: Vec<String>,
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> HashMap<String, Object> {
    let mut valves = HashMap::default();

    for line in input.lines() {
        let mut valve: String = String::new();
        let mut rate: usize = 0;
        let mut _connections: String = String::new();
        if line.contains("tunnels") {
            scanf::sscanf!(
                line,
                "Valve {} has flow rate={}; tunnels lead to valves {}",
                valve,
                rate,
                _connections
            )
        } else {
            scanf::sscanf!(
                line,
                "Valve {} has flow rate={}; tunnel leads to valve {}",
                valve,
                rate,
                _connections
            )
        }
        .unwrap();
        let connections = _connections.split(", ").map(|s| s.to_string()).collect();
        valves.insert(valve, Object { rate, connections });
    }

    valves
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State<'a> {
    open: HashSet<&'a str>,
    current_room: &'a str,
    flow_per: usize,
    total: usize,
}

impl<'a> State<'a> {
    fn next(&self, object: &'a HashMap<String, Object>) -> Vec<Self> {
        let mut res = vec![];
        let total = self.total + self.flow_per;

        // Open Valve:
        if !self.open.contains(self.current_room) && object[self.current_room].rate > 0 {
            let mut open = self.open.clone();
            open.insert(self.current_room);
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
                current_room: next_valve,
                total,
                open: self.open.clone(),
                flow_per: self.flow_per,
            });
        }
        res
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ElephantState<'a> {
    open: HashSet<&'a str>,
    current_room: &'a str,
    elephant_room: &'a str,
    flow_per: usize,
    total: usize,
}

impl<'a> ElephantState<'a> {
    fn next(&self, object: &'a HashMap<String, Object>) -> Vec<Self> {
        let mut vec_you = vec![];
        let total = self.total + self.flow_per;

        // Open Valve (you):
        if !self.open.contains(self.current_room) && object[self.current_room].rate > 0 {
            let mut open = self.open.clone();
            open.insert(self.current_room);
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
                    open: self.open.clone(),
                    current_room: next_valve,
                    elephant_room: self.elephant_room,
                    flow_per: self.flow_per,
                    total,
                },
                false,
            ));
        }

        let mut vec_elephant = vec![];
        if !self.open.contains(self.elephant_room) && object[self.elephant_room].rate > 0 {
            let mut open = self.open.clone();
            open.insert(self.elephant_room);
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
                    open: self.open.clone(),
                    current_room: self.current_room,
                    elephant_room: next_valve,
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
            let elphant_room = elephant.elephant_room;
            if you_opened && elephant_opened {
                if you.current_room != elephant.elephant_room {
                    let mut open = you.open.clone();
                    open.insert(self.elephant_room);

                    res.push(ElephantState {
                        open,
                        current_room,
                        elephant_room: elphant_room,
                        flow_per: you.flow_per + object[elephant.elephant_room].rate,
                        total,
                    });
                }
            } else {
                let open = if you.open.len() > elephant.open.len() {
                    you.open
                } else {
                    elephant.open
                };

                res.push(ElephantState {
                    open,
                    current_room,
                    elephant_room: elphant_room,
                    flow_per: you.flow_per.max(elephant.flow_per),
                    total,
                });
            }
        }
        res
    }
}

#[aoc(day16, part1)]
pub fn part1(inputs: &HashMap<String, Object>) -> usize {
    let mut state = vec![State {
        open: Default::default(),
        current_room: "AA",
        flow_per: 0,
        total: 0,
    }];
    for _ in 0..30 {
        state = state
            .into_iter()
            .flat_map(|state| state.next(inputs).into_iter())
            .sorted_by_key(|state| Reverse(state.total))
            .take(400)
            .collect();
    }

    state
        .into_iter()
        .max_by_key(|state| state.total)
        .unwrap()
        .total
}

#[aoc(day16, part2)]
pub fn part2(inputs: &HashMap<String, Object>) -> usize {
    let mut state = vec![ElephantState {
        open: Default::default(),
        current_room: "AA",
        elephant_room: "AA",
        flow_per: 0,
        total: 0,
    }];
    for _ in 0..26 {
        state = state
            .into_iter()
            .flat_map(|state| state.next(inputs).into_iter())
            .sorted_by_key(|state| Reverse(state.total))
            .take(3075)
            .collect();
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
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 1651);
    }

    #[test]
    pub fn test2() {
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
