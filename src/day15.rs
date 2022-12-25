use std::iter::repeat;

use ahash::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use scanf::sscanf;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    sensor: (i64, i64),
    beacon: (i64, i64),
    distance: i64,
}

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<Object> {
    input
        .lines()
        .map(|line| {
            let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
            sscanf!(
                line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                a,
                b,
                c,
                d
            )
            .unwrap();
            Object {
                sensor: (b, a),
                beacon: (d, c),
                distance: manhattan_distance((b, a), (d, c)),
            }
        })
        .collect()
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i64
}

#[aoc(day15, part1)]
pub fn part1(inputs: &[Object]) -> usize {
    let objects = inputs
        .iter()
        .flat_map(
            |&Object {
                 sensor,
                 beacon,
                 distance: _,
             }| [sensor, beacon],
        )
        .collect::<HashSet<_>>();

    let beacon_min_max = inputs
        .iter()
        .flat_map(
            |&Object {
                 sensor,
                 beacon: _,
                 distance,
             }| [sensor.1 - distance, sensor.1 + distance],
        )
        .minmax()
        .into_option()
        .unwrap();

    // println!("{beacon_min_max:?}");

    let y = if inputs.len() == 14 { 10 } else { 2000000 };
    let mut count = 0;
    for x in beacon_min_max.0..=beacon_min_max.1 {
        for sensor in inputs.iter() {
            if manhattan_distance((y, x), sensor.sensor) <= sensor.distance
                && !objects.contains(&(y, x))
            {
                // println!("({y},{x})");
                count += 1;
                break;
            }
        }
    }
    count

    // let mut map: HashMap<(i64, i64), u8> = HashMap::default();

    // for &Object {
    //     sensor,
    //     beacon,
    //     distance,
    // } in inputs.iter()
    // {
    //     map.insert(sensor, b'b');
    //     map.insert(beacon, b'b');

    //     for x in 0..=distance {
    //         for y in 0..=(distance - x) {
    //             *map.entry((sensor.0 + y, sensor.1 + x)).or_insert(0) += 1;
    //             *map.entry((sensor.0 + y, sensor.1 - x)).or_insert(0) += 1;
    //             *map.entry((sensor.0 - y, sensor.1 + x)).or_insert(0) += 1;
    //             *map.entry((sensor.0 - y, sensor.1 - x)).or_insert(0) += 1;
    //         }
    //     }
    // }

    // map.iter()
    //     .filter(|((y, _), v)| y == &10 && **v < 99)
    //     .count()
}

#[aoc(day15, part2)]
pub fn part2(inputs: &[Object]) -> i64 {
    let objects = inputs
        .iter()
        .flat_map(
            |&Object {
                 sensor,
                 beacon,
                 distance: _,
             }| [sensor, beacon],
        )
        .collect();

    let ans = scan(inputs, &objects).expect("not found");

    ans.1 * 4000000 + ans.0
}

fn scan(inputs: &[Object], objects: &HashSet<(i64, i64)>) -> Option<(i64, i64)> {
    let max = if inputs.len() == 14 { 20 } else { 4000000 };

    inputs.par_iter().find_map_any(|o| {
        let mut point = (o.sensor.0 - o.distance - 1, o.sensor.1);
        // for dir in [(1, 1), (1, -1), (-1, -1), (-1, 1)] {
        //     for _ in 0..o.distance {
        //         point = (point.0 + dir.0, point.1 + dir.1);
        //         if out_range(inputs, objects, point, max) {
        //             return Some(point);
        //         }
        //     }
        // }
        // None

        [(1, 1), (1, -1), (-1, -1), (-1, 1)]
            .iter()
            .flat_map(|x| repeat(x).take(o.distance as usize))
            .find_map(|dir| {
                point = (point.0 + dir.0, point.1 + dir.1);
                if out_range(inputs, objects, point, max) {
                    Some(point)
                } else {
                    None
                }
            })
    })
}

fn out_range(inputs: &[Object], objects: &HashSet<(i64, i64)>, p: (i64, i64), max: i64) -> bool {
    if p.0 > 0 && p.1 > 0 && p.0 <= max && p.1 <= max && !objects.contains(&p) {
        inputs.iter().all(
            |&Object {
                 sensor,
                 beacon: _,
                 distance,
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
