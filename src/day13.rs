use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::common::nom::{nom_u8, process_input};

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[inline]
fn signal(s: &str) -> IResult<&str, Signal> {
    alt((
        map(nom_u8, Signal::Value),
        map(
            delimited(tag("["), separated_list0(tag(","), signal), tag("]")),
            Signal::List,
        ),
    ))(s)
}

fn signals(s: &str) -> IResult<&str, [Signal; 2]> {
    map(separated_pair(signal, tag("\n"), signal), |(a, b)| [a, b])(s)
}

#[aoc_generator(day13)]
pub fn generator(inputs: &str) -> Vec<[Signal; 2]> {
    process_input(separated_list0(tag("\n\n"), signals))(inputs)
}

#[aoc(day13, part1)]
pub fn part1(input: &[[Signal; 2]]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, [x, y])| if x <= y { Some(i + 1) } else { None })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[[Signal; 2]]) -> usize {
    // let a = signal("[[2]]").unwrap().1;
    // let b = signal("[[6]]").unwrap().1;
    let a = Signal::List(vec![Signal::List(vec![Signal::Value(2)])]);
    let b = Signal::List(vec![Signal::List(vec![Signal::Value(6)])]);

    let (x, y) = input
        .iter()
        .flatten()
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
    pub fn input_test() {
        // println!("{:?}", generator1(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 13);
    }

    #[test]
    pub fn part2_test() {
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
