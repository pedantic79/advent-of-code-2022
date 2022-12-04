use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::utils::parse_split_n;

type Range = (u32, u32);

#[derive(Debug, PartialEq, Eq)]
pub struct Assignments {
    one: Range,
    two: Range,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Assignments> {
    input
        .lines()
        .map(|line| {
            let [a, b, x, y] = parse_split_n(line, &[',', '-'][..]).unwrap();
            let (one, two) = ((a, b), (x, y));
            Assignments { one, two }
        })
        .collect()
}

fn overlap1(a: Range, b: Range) -> bool {
    (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)
}

fn overlap2(a: Range, b: Range) -> bool {
    a.0 <= b.1 && a.1 >= b.0
}

fn solve<F: Fn(Range, Range) -> bool>(assignments: &[Assignments], pred: F) -> usize {
    assignments
        .iter()
        .filter(|Assignments { one, two }| pred(*one, *two))
        .count()
}

#[aoc(day4, part1)]
pub fn part1(assignments: &[Assignments]) -> usize {
    solve(assignments, overlap1)
}

#[aoc(day4, part2)]
pub fn part2(assignments: &[Assignments]) -> usize {
    solve(assignments, overlap2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 4);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day4.txt");
        const ANSWERS: (usize, usize) = (464, 770);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
