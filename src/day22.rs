mod part1;
mod part2;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{complete, map},
    multi::many0,
    IResult, Parser,
};

use crate::common::nom::nom_u32;

#[derive(Debug, PartialEq, Eq)]
enum Moves {
    Forward(u32),
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn value(&self) -> usize {
        match self {
            Dir::Left => 2,
            Dir::Right => 0,
            Dir::Up => 3,
            Dir::Down => 1,
        }
    }

    fn increment(&self, n: usize) -> Self {
        let n = (self.value() + n) % 4;
        match n % 4 {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => unreachable!(),
        }
    }

    fn turn(&self, m: &Moves) -> Self {
        match m {
            Moves::Forward(_) => panic!("can't turn a Forward"),
            Moves::Right => self.increment(1),
            Moves::Left => self.increment(3),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    mv: Vec<Moves>,
    board: Vec<Vec<u8>>,
}

impl Input {
    fn find_start(&self) -> (usize, usize) {
        (0, self.board[0].iter().position(|&x| x == b'.').unwrap())
    }

    fn board_get(&self, pos: (usize, usize)) -> Option<u8> {
        let row = self.board.get(pos.0)?;
        let cell = row.get(pos.1)?;

        if *cell == b' ' {
            None
        } else {
            Some(*cell)
        }
    }
}

fn parse_moves(line: &str) -> IResult<&str, Vec<Moves>> {
    complete(many0(alt((
        map(tag("L"), |_| Moves::Left),
        map(tag("R"), |_| Moves::Right),
        map(nom_u32, Moves::Forward),
    ))))
    .parse(line)
}

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Input {
    let mut iter = input.lines().rev();
    let mv = parse_moves(iter.next().unwrap()).unwrap().1;
    iter.next().unwrap();
    let board = iter.rev().map(|l| l.to_string().into_bytes()).collect();

    Input { mv, board }
}

fn solve<F: Fn(&Input, &mut Dir, u32, (usize, usize)) -> (usize, usize)>(
    inputs: &Input,
    move_forward: F,
) -> usize {
    let mut pos = inputs.find_start();
    let mut direction = Dir::Right;
    // println!("start: {pos:?}");

    for m in &inputs.mv {
        match m {
            Moves::Forward(n) => {
                pos = move_forward(inputs, &mut direction, *n, pos);
                // println!("move foward {n}, new: {pos:?}");
            }
            Moves::Right => {
                direction = direction.turn(&Moves::Right);
                // println!("turn right: Now {direction:?}")
            }
            Moves::Left => {
                direction = direction.turn(&Moves::Left);
                // println!("turn left: Now {direction:?}")
            }
        }
    }

    (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + direction.value()
}

#[aoc(day22, part1)]
pub fn part1(inputs: &Input) -> usize {
    solve(inputs, part1::move_forward)
}

#[aoc(day22, part2)]
pub fn part2(inputs: &Input) -> usize {
    solve(inputs, part2::move_forward)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("day22/SAMPLE.txt");

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 6032);
    }

    #[test]
    pub fn part2_test() {
        // assert_eq!(part2(&generator(SAMPLE)), 5031);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day22.txt");
        const ANSWERS: (usize, usize) = (26558, 110400);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
