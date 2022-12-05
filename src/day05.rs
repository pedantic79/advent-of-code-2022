use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Map {
    stacks: Vec<Vec<u8>>,
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = (s.lines().rev().next().unwrap().len() + 2) / 4;

        let mut stacks = vec![Vec::new(); width];

        for line in s.lines().rev().skip(1) {
            if line.is_empty() {
                continue;
            }
            let line = &line.as_bytes()[1..];
            for (index, s) in line.chunks(4).enumerate() {
                if s[0] != b' ' {
                    stacks[index].push(s[0])
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

impl FromStr for Move {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut count = 0;
        let mut from_stack = 0;
        let mut to_stack = 0;

        scanf::sscanf!(s, "move {} from {} to {}", count, from_stack, to_stack).unwrap();
        Ok(Self {
            count,
            from_stack: from_stack - 1,
            to_stack: to_stack - 1,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Input {
    map: Map,
    moves: Vec<Move>,
}

fn process_moves2(moves: &[Move], map: &mut Map) {
    let mut temp = Vec::new();
    for &m in moves {
        let count = m.count;
        let start = map.stacks[m.from_stack].len() - count;
        temp.extend(map.stacks[m.from_stack].drain(start..));
        map.stacks[m.to_stack].append(&mut temp);
    }
}

impl Input {
    fn process_moves(&mut self) {
        for &m in &self.moves {
            let count = m.count;
            for _ in 0..count {
                if let Some(temp) = self.map.stacks[m.from_stack].pop() {
                    self.map.stacks[m.to_stack].push(temp);
                }
            }
        }
    }

    fn get_anser(&self) -> String {
        let mut ans = String::new();
        for s in &self.map.stacks {
            ans.push(*s.last().unwrap_or(&b' ') as char);
        }
        ans
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Input {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map = map.parse().unwrap();
    let moves = moves.lines().map(|line| line.parse().unwrap()).collect();

    Input { map, moves }
}

#[aoc(day5, part1)]
pub fn part1(inputs: &Input) -> String {
    let mut inputs = inputs.clone();
    inputs.process_moves();
    inputs.get_anser()
}

#[aoc(day5, part2)]
pub fn part2(inputs: &Input) -> String {
    let mut inputs = inputs.clone();
    process_moves2(&inputs.moves, &mut inputs.map);
    inputs.get_anser()
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
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), "CMZ");
    }

    #[test]
    pub fn test2() {
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
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
