use ahash::HashSetExt;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{self, tag},
    combinator::map,
    sequence::tuple,
};
use rustc_hash::FxHashSet as HashSet;

use crate::common::nom::{nom_lines, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    dir: u8,
    mag: usize,
}

impl Move {
    fn get(&self) -> (isize, isize, usize) {
        let x = self.mag;
        match self.dir {
            b'U' => (1, 0, x),
            b'D' => (-1, 0, x),
            b'R' => (0, 1, x),
            b'L' => (0, -1, x),
            _ => panic!("invalid move"),
        }
    }
}

#[derive(Debug)]
pub struct Snake<const N: usize> {
    rope: [(isize, isize); N],
    pos: HashSet<(isize, isize)>,
}

impl<const N: usize> Default for Snake<N> {
    fn default() -> Self {
        Self {
            rope: [Default::default(); N],
            pos: HashSet::with_capacity(8192),
        }
    }
}

impl<const N: usize> Snake<N> {
    pub fn process_move(&mut self, m: &Move) {
        let (r, c, mag) = m.get();
        self.pos.insert(self.rope[N - 1]);

        'outer: for _ in 0..mag {
            self.rope[0] = (self.rope[0].0 + r, self.rope[0].1 + c);

            for x in 1..N {
                if !self.update_tail(x, self.rope[x - 1]) {
                    // not updated
                    continue 'outer;
                    // break;
                }
            }

            self.pos.insert(self.rope[N - 1]);
        }
    }

    pub fn update_tail(&mut self, pos: usize, last: (isize, isize)) -> bool {
        let d = (last.0 - self.rope[pos].0, last.1 - self.rope[pos].1);

        if d.0.abs() > 1 || d.1.abs() > 1 {
            self.rope[pos].0 += d.0.signum();
            self.rope[pos].1 += d.1.signum();
            true
        } else {
            false
        }
    }
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Move> {
    process_input(nom_lines(map(
        tuple((complete::take(1_usize), tag(" "), nom_usize::<&str>)),
        |(dir, _, mag)| Move {
            dir: dir.as_bytes()[0],
            mag,
        },
    )))(input)
}

fn solve<const N: usize>(inputs: &[Move]) -> usize {
    let mut snake = Snake::<N>::default();

    for m in inputs {
        snake.process_move(m);
    }

    snake.pos.len()
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[Move]) -> usize {
    solve::<2>(inputs)
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[Move]) -> usize {
    solve::<10>(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const SAMPLE2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 13);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 1);
        assert_eq!(part2(&generator(SAMPLE2)), 36);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day9.txt");
        const ANSWERS: (usize, usize) = (5683, 2372);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
