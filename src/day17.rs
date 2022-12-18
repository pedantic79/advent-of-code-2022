use std::{collections::hash_map, fmt::Debug};

use ahash::HashMapExt;
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap as HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Vec<Direction> {
    input
        .bytes()
        .map(|b| {
            if b == b'<' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect()
}

struct Chamber(Vec<u8>);

impl Debug for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", ChamberView(&self.0))
    }
}

fn display(b: u8) -> char {
    if b > 0 {
        '#'
    } else {
        '.'
    }
}

struct ChamberView<'a>(&'a [u8]);

impl<'a> Debug for ChamberView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in self.0.iter().rev() {
            writeln!(
                f,
                "|{}{}{}{}{}{}{}|",
                display(row & 1 << 6),
                display(row & 1 << 5),
                display(row & 1 << 4),
                display(row & 1 << 3),
                display(row & 1 << 2),
                display(row & 1 << 1),
                display(row & 1),
            )?;
        }

        writeln!(f, "+-------+\n")
    }
}

impl Chamber {
    fn piece2u8(piece: u32, row: usize, x: usize) -> u8 {
        ((piece >> (u8::BITS as usize * row) & 0xff) >> (x + 1)) as u8
    }

    fn check_piece(&self, piece: u32, y: usize, x: usize) -> bool {
        (0..4).all(|row| Self::piece2u8(piece, row, x) & self.0[y + row] == 0)
    }

    fn write_piece(&mut self, piece: u32, dy: usize, dx: usize) {
        for (row, grid_row) in self.0[dy..(dy + 4)].iter_mut().enumerate() {
            *grid_row |= Self::piece2u8(piece, row, dx);
        }
    }
}

enum Piece {
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}

impl Piece {
    const fn dimensions(&self) -> (usize, usize) {
        match self {
            Piece::Horizontal => (1, 4),
            Piece::Plus => (3, 3),
            Piece::Corner => (3, 3),
            Piece::Vertical => (4, 1),
            Piece::Square => (2, 2),
        }
    }

    const fn piece(&self) -> u32 {
        #[allow(clippy::unusual_byte_groupings)]
        match self {
            Piece::Horizontal => 0b00000000_00000000_00000000_11110000,
            Piece::Plus => 0b00000000_01000000_11100000_01000000,
            Piece::Corner => 0b00000000_00100000_00100000_11100000,
            Piece::Vertical => 0b10000000_10000000_10000000_10000000,
            Piece::Square => 0b00000000_00000000_11000000_11000000,
        }
    }

    const fn get_info(&self) -> (u32, usize, usize) {
        let (h, w) = self.dimensions();
        (self.piece(), h, w)
    }
}

#[aoc(day17, part1)]
pub fn part1(inputs: &[Direction]) -> usize {
    let mut chamber = Chamber(vec![0; 1024 * 4]);
    let mut drafts = inputs.iter().cycle();
    let mut max = 0;

    for (_, &(piece, piece_height, piece_width)) in (0..2022).zip(
        [
            Piece::Horizontal.get_info(),
            Piece::Plus.get_info(),
            Piece::Corner.get_info(),
            Piece::Vertical.get_info(),
            Piece::Square.get_info(),
        ]
        .iter()
        .cycle(),
    ) {
        let mut y = max + 3;
        let mut x = 2;
        loop {
            // Move Left/Right
            let draft = drafts.next().unwrap();
            match draft {
                Direction::Left => {
                    if x > 0 && chamber.check_piece(piece, y, x - 1) {
                        x -= 1;
                    }
                }
                Direction::Right => {
                    if x + piece_width < 7 && chamber.check_piece(piece, y, x + 1) {
                        x += 1
                    }
                }
            }

            // Move Down
            if y > 0 && chamber.check_piece(piece, y - 1, x) {
                y -= 1;
            } else {
                break;
            }
        }
        chamber.write_piece(piece, y, x);
        max = max.max(y + piece_height);
        // println!("{chamber:?}\n");
    }

    max
}

#[aoc(day17, part2)]
pub fn part2(inputs: &[Direction]) -> usize {
    let mut chamber = Chamber(vec![0; 1024 * 5]);
    let mut drafts = inputs.iter().enumerate().cycle();
    let mut max = 0;
    let mut seen = HashMap::with_capacity(500);

    for (iteration, (piece_index, &(piece, piece_height, piece_width))) in (1..).zip(
        [
            Piece::Horizontal.get_info(),
            Piece::Plus.get_info(),
            Piece::Corner.get_info(),
            Piece::Vertical.get_info(),
            Piece::Square.get_info(),
        ]
        .iter()
        .enumerate()
        .cycle(),
    ) {
        let mut y = max + 3;
        let mut x = 2;
        let draft_index = loop {
            // Move Left/Right
            let (draft_index, draft) = drafts.next().unwrap();
            match draft {
                Direction::Left => {
                    if x > 0 && chamber.check_piece(piece, y, x - 1) {
                        x -= 1;
                    }
                }
                Direction::Right => {
                    if x + piece_width < 7 && chamber.check_piece(piece, y, x + 1) {
                        x += 1
                    }
                }
            }

            // Move Down
            if y > 0 && chamber.check_piece(piece, y - 1, x) {
                y -= 1;
            } else {
                break draft_index;
            }
        };
        chamber.write_piece(piece, y, x);
        max = max.max(y + piece_height);

        const MAX_ITERATIONS: usize = 1_000_000_000_000;
        if piece_index == 4 {
            match seen.entry(draft_index) {
                hash_map::Entry::Occupied(x) => {
                    let &(i, m) = x.get();
                    let iteration_period = iteration - i;
                    let height_period = max - m;

                    if MAX_ITERATIONS % iteration_period == iteration % iteration_period {
                        let num_remaining = (MAX_ITERATIONS - iteration) / iteration_period;
                        return max + num_remaining * height_period;
                    }
                }
                hash_map::Entry::Vacant(x) => {
                    x.insert((iteration, max));
                }
            };
        }

        // println!("{chamber:?}\n");
    }

    panic!("shouldn't get here")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 3068);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 1514285714288);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day17.txt");
        const ANSWERS: (usize, usize) = (3161, 1575931232076);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
