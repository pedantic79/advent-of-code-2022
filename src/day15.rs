use std::iter;

use ahash::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{IResult, Parser, bytes::complete::tag, combinator::map};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::common::nom::{nom_i64, nom_lines, process_input};

#[derive(Debug, PartialEq, Eq)]
pub struct SensorReport {
    sensor: (i64, i64),
    beacon: (i64, i64),
    distance: i64,
}

fn parse_sensor_report(line: &str) -> IResult<&str, SensorReport> {
    map(
        (
            tag("Sensor at x="),
            nom_i64,
            tag(", y="),
            nom_i64,
            tag(": closest beacon is at x="),
            nom_i64,
            tag(", y="),
            nom_i64,
        ),
        |(_, a, _, b, _, c, _, d)| SensorReport {
            sensor: (b, a),
            beacon: (d, c),
            distance: manhattan_distance((b, a), (d, c)),
        },
    )
    .parse(line)
}

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<SensorReport> {
    process_input(nom_lines(parse_sensor_report))(input)
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i64
}

#[aoc(day15, part1)]
pub fn part1(reports: &[SensorReport]) -> usize {
    let objects = reports
        .iter()
        .flat_map(|&SensorReport { sensor, beacon, .. }| [sensor, beacon])
        .collect::<HashSet<_>>();

    let beacon_min_max = reports
        .iter()
        .flat_map(
            |&SensorReport {
                 sensor, distance, ..
             }| [sensor.1 - distance, sensor.1 + distance],
        )
        .minmax()
        .into_option()
        .unwrap();

    let y = if reports.len() == 14 { 10 } else { 2000000 };

    (beacon_min_max.0..=beacon_min_max.1)
        .into_par_iter()
        .filter_map(|x| {
            reports.iter().find(|sensor| {
                manhattan_distance((y, x), sensor.sensor) <= sensor.distance
                    && !objects.contains(&(y, x))
            })
        })
        .count()
}

#[aoc(day15, part2)]
pub fn part2(reports: &[SensorReport]) -> i64 {
    let poi = reports
        .iter()
        .flat_map(|&SensorReport { sensor, beacon, .. }| [sensor, beacon])
        .collect();

    let ans = scan(reports, &poi).expect("not found");

    ans.1 * 4000000 + ans.0
}

fn scan(inputs: &[SensorReport], points_of_interest: &HashSet<(i64, i64)>) -> Option<(i64, i64)> {
    let max = if inputs.len() == 14 { 20 } else { 4000000 };

    inputs.par_iter().find_map_any(|o| {
        let mut point = (o.sensor.0 - o.distance - 1, o.sensor.1);

        [(1, 1), (1, -1), (-1, -1), (-1, 1)]
            .iter()
            .flat_map(|x| iter::repeat_n(x, o.distance as usize))
            .find_map(|dir| {
                point = (point.0 + dir.0, point.1 + dir.1);
                if out_range(inputs, points_of_interest, point, max) {
                    Some(point)
                } else {
                    None
                }
            })
    })
}

fn out_range(
    inputs: &[SensorReport],
    points_of_interest: &HashSet<(i64, i64)>,
    p: (i64, i64),
    max: i64,
) -> bool {
    if p.0 > 0 && p.1 > 0 && p.0 <= max && p.1 <= max && !points_of_interest.contains(&p) {
        inputs.iter().all(
            |&SensorReport {
                 sensor, distance, ..
             }| manhattan_distance(p, sensor) > distance,
        )
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 26);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 56000011);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day15.txt");
        const ANSWERS: (usize, i64) = (6275922, 11747175442119);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
