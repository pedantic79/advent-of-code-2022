use crate::common::{
    nom::{nom_lines, nom_usize, process_input},
    GetMutTwice,
};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, combinator::map};
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Crates {
    stacks: Vec<Vec<u8>>,
}

impl FromStr for Crates {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();
        let width = (lines.next().unwrap().len() + 2) / 4;
        let mut stacks = vec![Vec::with_capacity(64); width];

        for line in lines {
            let line = line.as_bytes();
            for (name, stack) in line.chunks(4).zip(stacks.iter_mut()) {
                if name[1] != b' ' {
                    stack.push(name[1])
                }
            }
        }

        Ok(Self { stacks })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move {
    count: usize,
    from_stack: usize,
    to_stack: usize,
}

fn parse_moves(s: &str) -> Vec<Move> {
    process_input(nom_lines(map(
        (
            tag("move "),
            nom_usize,
            tag(" from "),
            nom_usize,
            tag(" to "),
            nom_usize,
        ),
        |(_, count, _, from_stack, _, to_stack)| Move {
            count,
            from_stack: from_stack - 1,
            to_stack: to_stack - 1,
        },
    )))(s)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Input {
    crates: Crates,
    moves: Vec<Move>,
}

impl Input {
    fn process_moves1(&mut self) {
        for &m in &self.moves {
            let start = self.crates.stacks[m.from_stack].len() - m.count;
            let (f, t) = self.crates.stacks.get_mut_twice(m.from_stack, m.to_stack);
            t.extend(f.drain(start..).rev());
        }
    }

    fn process_moves2(&mut self) {
        for &m in &self.moves {
            let start = self.crates.stacks[m.from_stack].len() - m.count;
            let (f, t) = self.crates.stacks.get_mut_twice(m.from_stack, m.to_stack);
            t.extend(f.drain(start..));
        }
    }

    fn read_tops(&self) -> String {
        let v = self
            .crates
            .stacks
            .iter()
            .map(|s| *s.last().unwrap_or(&b' '))
            .collect();

        // SAFETY: We're only dealing with ascii
        unsafe { String::from_utf8_unchecked(v) }
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Input {
    let (crates, moves) = input.split_once("\n\n").unwrap();
    let crates = crates.trim_end().parse().unwrap();
    let moves = parse_moves(moves);

    Input { crates, moves }
}

#[aoc(day5, part1)]
pub fn part1(inputs: &Input) -> String {
    let mut inputs = inputs.clone();
    inputs.process_moves1();
    inputs.read_tops()
}

#[aoc(day5, part2)]
pub fn part2(inputs: &Input) -> String {
    let mut inputs = inputs.clone();
    inputs.process_moves2();
    inputs.read_tops()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), "CMZ");
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), "MCD");
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day5.txt");
        const ANSWERS: (&str, &str) = ("HNSNMTLHQ", "RNLFDJMCT");

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
