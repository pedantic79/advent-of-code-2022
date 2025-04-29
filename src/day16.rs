use std::collections::VecDeque;

use ahash::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::IndexSet;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    multi::separated_list1,
};

use crate::common::nom::{nom_lines, nom_u32, process_input};

#[derive(Debug, PartialEq, Eq)]
pub struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    tunnels: Vec<&'a str>,
}

fn parse_line(s: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    let (s, (_, name, _, rate, _, tunnels)) = (
        tag("Valve "),
        take(2usize),
        tag(" has flow rate="),
        nom_u32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), take(2usize)),
    )
        .parse(s)?;

    Ok((s, (name, rate, tunnels)))
}

fn parse_input_value(s: &str) -> IResult<&str, Valve<'_>> {
    let (s, (name, flow_rate, tunnels)) = parse_line(s)?;

    Ok((
        s,
        Valve {
            name,
            flow_rate,
            tunnels,
        },
    ))
}

#[derive(Debug)]
pub struct Graph {
    flow_rates: Vec<u32>,
    target_valve_mapping: Vec<(usize, usize)>,
    distances: Vec<Vec<u32>>,
}

impl Graph {
    fn from_valves(valves: &[Valve<'_>]) -> Self {
        let flow_rates = valves.iter().map(|v| v.flow_rate).collect_vec();

        let mut target_valve_mapping = Vec::new();
        for (id, &rate) in flow_rates.iter().enumerate() {
            if rate > 0 {
                let target_index = target_valve_mapping.len();
                target_valve_mapping.push((id, target_index));
            }
        }

        let distances = calculate_distance(valves, &target_valve_mapping);

        Graph {
            flow_rates,
            target_valve_mapping,
            distances,
        }
    }
}

fn bfs(start: usize, distances: &mut [u32], adj_map: &HashMap<usize, Vec<usize>>) {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0)); // (node_id, distance)
    distances[start] = 0;

    while let Some((current_id, dist)) = queue.pop_front() {
        if let Some(neighbors) = adj_map.get(&current_id) {
            for &neighbor_id in neighbors {
                if distances[neighbor_id] == u32::MAX {
                    distances[neighbor_id] = dist + 1;
                    queue.push_back((neighbor_id, distances[neighbor_id]));
                }
            }
        }
    }
}

fn calculate_distance(
    valves: &[Valve<'_>],
    target_valve_mapping: &[(usize, usize)],
) -> Vec<Vec<u32>> {
    const START: usize = 0;

    let num_nodes = valves.len();

    let lookup = valves
        .iter()
        .map(|v| v.name)
        .collect::<IndexSet<_, ahash::RandomState>>();

    let adj_map = valves
        .iter()
        .map(|valve| {
            let id = lookup.get_index_of(valve.name).unwrap();
            let neighbors = valve
                .tunnels
                .iter()
                .map(|&name| lookup.get_index_of(name).unwrap())
                .collect();

            (id, neighbors)
        })
        .collect();

    // Nodes relevant for distance calculation: Start node + Target nodes
    let mut relevant_nodes = Vec::with_capacity(target_valve_mapping.len() + 1);
    if target_valve_mapping[0].0 != START {
        relevant_nodes.push(START);
    }
    relevant_nodes.extend(target_valve_mapping.iter().map(|x| x.0));

    // Calculate all-pairs shortest paths between relevant nodes using pathfinding::prelude::bfs
    let mut distances = vec![vec![u32::MAX; num_nodes]; num_nodes];
    for &node_id in &relevant_nodes {
        bfs(node_id, &mut distances[node_id], &adj_map);
    }

    distances
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Graph {
    // Ensure that "AA" is always the first valve
    fn move_start_to_front(mut valves: Vec<Valve<'_>>) -> Vec<Valve<'_>> {
        let idx = valves.iter().position(|v| v.name == "AA").unwrap();
        valves.swap(idx, 0);
        valves
    }

    let valves = move_start_to_front(process_input(nom_lines(parse_input_value))(input));

    Graph::from_valves(&valves)
}

fn dfs_part1(
    id: usize,
    time_left: u32,
    mask: u64,
    graph: &Graph,
    memo: &mut HashMap<(usize, u32, u64), u32>,
) -> u32 {
    let memo_key = (id, time_left, mask);
    if let Some(&cached_pressure) = memo.get(&memo_key) {
        return cached_pressure;
    }

    let mut max_pressure = 0;

    // Iterate through all potential target valves to open next
    for &(next_target_id, target_idx) in &graph.target_valve_mapping {
        // Check if this target valve is already open
        if (mask >> target_idx) & 1 == 1 {
            continue;
        }

        let time_to_move = graph.distances[id][next_target_id];
        if time_to_move == u32::MAX {
            // Skip if unreachable (safety check)
            continue;
        }

        let time_needed = time_to_move + 1;

        if time_left >= time_needed {
            let new_time_left = time_left - time_needed;
            let pressure_gain = graph.flow_rates[next_target_id] * new_time_left;

            let remaining_pressure = dfs_part1(
                next_target_id,
                new_time_left,
                mask | (1 << target_idx),
                graph,
                memo,
            );

            max_pressure = max_pressure.max(pressure_gain + remaining_pressure);
        }
    }

    memo.insert(memo_key, max_pressure);
    max_pressure
}

fn dfs_part2(
    id: usize,
    time_left: u32,
    mask: u64,
    current_total_pressure: u32,
    graph: &Graph,
    max_pressure_for_mask: &mut HashMap<u64, u32>,
) {
    // Update the maximum pressure recorded for this specific set of open valves (mask)
    max_pressure_for_mask
        .entry(mask)
        .and_modify(|p| *p = (*p).max(current_total_pressure))
        .or_insert(current_total_pressure);

    // Iterate through all potential target valves to open next
    for &(next_target_id, target_idx) in &graph.target_valve_mapping {
        // Check if this target valve is already open in the current path
        if (mask >> target_idx) & 1 == 1 {
            continue;
        }

        let time_to_move = graph.distances[id][next_target_id];
        if time_to_move == u32::MAX {
            // Skip if unreachable (safety check)
            continue;
        }

        let time_needed = time_to_move + 1;

        if time_left >= time_needed {
            let new_time_left = time_left - time_needed;
            let pressure_gain = graph.flow_rates[next_target_id] * new_time_left;

            dfs_part2(
                next_target_id,
                new_time_left,
                mask | (1 << target_idx),
                current_total_pressure + pressure_gain,
                graph,
                max_pressure_for_mask,
            );
        }
    }
}

#[aoc(day16, part1)]
pub fn part1(graph: &Graph) -> u32 {
    dfs_part1(0, 30, 0, graph, &mut Default::default())
}

#[aoc(day16, part2)]
pub fn part2(graph: &Graph) -> u32 {
    let mut max_pressure_for_mask = HashMap::default();
    dfs_part2(0, 26, 0, 0, graph, &mut max_pressure_for_mask);

    // calculate the maximum pressure, where the masks are not the same
    max_pressure_for_mask
        .iter()
        .tuple_combinations()
        .filter_map(|((mask1, pressure1), (mask2, pressure2))| {
            if (mask1 & mask2) == 0 {
                Some(pressure1 + pressure2)
            } else {
                None
            }
        })
        .max()
        .unwrap()
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
        const ANSWERS: (u32, u32) = (1376, 1933);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
