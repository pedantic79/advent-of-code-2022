use std::collections::VecDeque;

use ahash::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

use crate::common::nom::nom_i64;
// use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

fn num(n: &str) -> IResult<&str, i64> {
    nom_i64(n)
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> HashSet<Cube> {
    input
        .lines()
        .map(|line| {
            map(tuple((num, tag(","), num, tag(","), num)), |v| Cube {
                x: v.0,
                y: v.2,
                z: v.4,
            })(line)
            .unwrap()
            .1
        })
        .collect()
}

fn adj(c: Cube) -> [Cube; 6] {
    [
        Cube { x: c.x + 1, ..c },
        Cube { x: c.x - 1, ..c },
        Cube { y: c.y + 1, ..c },
        Cube { y: c.y - 1, ..c },
        Cube { z: c.z + 1, ..c },
        Cube { z: c.z - 1, ..c },
    ]
}

fn solve(inputs: &HashSet<Cube>) -> usize {
    let mut count = 0;
    for cube in inputs {
        for a in adj(*cube) {
            if !inputs.contains(&a) {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day18, part1)]
pub fn part1(inputs: &HashSet<Cube>) -> usize {
    solve(inputs)
}

#[aoc(day18, part2)]
pub fn part2(inputs: &HashSet<Cube>) -> usize {
    let mut air = iproduct!(0..=19, 0..=19, 0..=19)
        .map(|(x, y, z)| Cube { x, y, z })
        .collect::<HashSet<_>>()
        .difference(inputs)
        .copied()
        .collect::<HashSet<_>>();

    // flood fill to remove external part
    let mut queue = VecDeque::new();
    queue.push_back(Cube { x: 0, y: 0, z: 0 });
    while let Some(c) = queue.pop_front() {
        if air.remove(&c) {
            queue.extend(adj(c));
        }
    }

    solve(inputs) - solve(&air)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 64);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 58);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day18.txt");
        const ANSWERS: (usize, usize) = (3494, 2062);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
