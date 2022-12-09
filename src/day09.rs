use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Up(isize),
    Down(isize),
    Right(isize),
    Left(isize),
}

impl FromStr for Move {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Move::*;
        let x = s[2..].parse().unwrap();
        Ok(match s.as_bytes()[0] {
            b'U' => Up(x),
            b'D' => Down(x),
            b'R' => Right(x),
            b'L' => Left(x),
            _ => panic!("unknown byte"),
        })
    }
}

#[derive(Debug)]
pub struct Snake<const N: usize> {
    head: (isize, isize),
    rope: [(isize, isize); N],
    pos: rustc_hash::FxHashSet<(isize, isize)>,
}

impl<const N: usize> Default for Snake<N> {
    fn default() -> Self {
        Self {
            head: Default::default(),
            rope: [Default::default(); N],
            pos: Default::default(),
        }
    }
}

impl<const N: usize> Snake<N> {
    pub fn process_move(&mut self, m: &Move) {
        let (r, c, mag) = match m {
            Move::Up(x) => (1, 0, *x),
            Move::Down(x) => (-1, 0, *x),
            Move::Right(x) => (0, 1, *x),
            Move::Left(x) => (0, -1, *x),
        };

        for _ in 0..mag {
            self.head = (self.head.0 + r, self.head.1 + c);

            let mut last = self.head;
            for x in 0..N {
                self.update_tail(x, last);
                last = self.rope[x];
            }
        }
    }

    pub fn update_tail(&mut self, pos: usize, last: (isize, isize)) {
        let d = (last.0 - self.rope[pos].0, last.1 - self.rope[pos].1);

        if d.0.abs() > 1 || d.1.abs() > 1 {
            self.rope[pos].0 += d.0.signum();
            self.rope[pos].1 += d.1.signum();
        }

        if pos == N - 1 {
            self.pos.insert(self.rope[pos]);
        }
    }
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Move> {
    input.lines().map(|l| l.parse().unwrap()).collect()
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
    solve::<1>(inputs)
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[Move]) -> usize {
    solve::<9>(inputs)
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
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 13);
    }

    #[test]
    pub fn test2() {
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
