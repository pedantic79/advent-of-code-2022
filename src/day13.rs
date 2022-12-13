use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Signal {
    List(Vec<Signal>),
    Value(u8),
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Signal::Value(a), Signal::Value(b)) => a.cmp(b),
            (Signal::Value(a), Signal::List(b)) => [Signal::Value(*a)][..].cmp(b),
            (Signal::List(a), Signal::Value(b)) => a[..].cmp(&[Signal::Value(*b)][..]),
            (Signal::List(a), Signal::List(b)) => a.cmp(b),
        }
    }
}

impl FromStr for Signal {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s).unwrap())
    }
}

#[aoc_generator(day13)]
pub fn generator(inputs: &str) -> Vec<Signal> {
    inputs
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[Signal]) -> usize {
    input
        .chunks(2)
        .enumerate()
        .filter_map(|(i, x)| if x[0] <= x[1] { Some(i + 1) } else { None })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[Signal]) -> usize {
    // let a = signal("[[2]]").unwrap().1;
    // let b = signal("[[6]]").unwrap().1;
    let a = Signal::List(vec![Signal::List(vec![Signal::Value(2)])]);
    let b = Signal::List(vec![Signal::List(vec![Signal::Value(6)])]);

    let (x, y) = input
        .iter()
        .fold((1, 2), |(mut count_a, mut count_b), sig| {
            if sig < &b {
                count_b += 1;
                if sig < &a {
                    count_a += 1;
                }
            }

            (count_a, count_b)
        });

    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator1(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 13);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 140);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day13.txt");
        const ANSWERS: (usize, usize) = (5852, 24190);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
