use ahash::HashMapExt;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;

const ALL: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const NORTH: [(i32, i32); 3] = [(-1, -1), (-1, 0), (-1, 1)];
const SOUTH: [(i32, i32); 3] = [(1, -1), (1, 0), (1, 1)];
const WEST: [(i32, i32); 3] = [(-1, -1), (0, -1), (1, -1)];
const EAST: [(i32, i32); 3] = [(-1, 1), (0, 1), (1, 1)];

const EMPTY: u8 = b'.';
const ELF: u8 = b'#';
const WIDTH: usize = 256;
const OFFSET: usize = WIDTH / 2;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Elf(usize, usize);

impl Elf {
    fn tick<A: AsRef<[u8]>>(&self, map: &[A], i: usize) -> Option<Elf> {
        if self.check(map, &ALL) {
            return None;
        }

        [
            (NORTH, (-1, 0)),
            (SOUTH, (1, 0)),
            (WEST, (0, -1)),
            (EAST, (0, 1)),
            (NORTH, (-1, 0)),
            (SOUTH, (1, 0)),
            (WEST, (0, -1)),
            (EAST, (0, 1)),
        ]
        .iter()
        .skip(i % 4)
        .take(4)
        .find_map(|(dir, delta)| {
            if self.check(map, dir) {
                Some(self.apply_delta(*delta))
            } else {
                None
            }
        })
    }

    fn check<A: AsRef<[u8]>>(&self, map: &[A], dirs: &[(i32, i32)]) -> bool {
        dirs.iter()
            .map(|delta| self.apply_delta(*delta))
            .all(|Elf(r, c)| map[r].as_ref()[c] == EMPTY)
    }

    fn apply_delta(&self, delta: (i32, i32)) -> Self {
        Self(
            (self.0 as i32 + delta.0) as usize,
            (self.1 as i32 + delta.1) as usize,
        )
    }
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> ([[u8; WIDTH]; WIDTH], Vec<Elf>) {
    let mut map = [[EMPTY; WIDTH]; WIDTH];
    let mut elves = Vec::new();
    for (r_idx, row) in input.lines().enumerate() {
        for (c_idx, cell) in row.bytes().enumerate() {
            if cell == ELF {
                map[r_idx + OFFSET][c_idx + OFFSET] = ELF;
                elves.push(Elf(r_idx + OFFSET, c_idx + OFFSET));
            }
        }
    }

    (map, elves)
}

fn solve<const ROUNDS: usize, N>(
    a: &[[u8; WIDTH]; WIDTH],
    elves: &[Elf],
    ret: impl Fn(usize, &[Elf]) -> N,
) -> N {
    let mut a = *a;
    let mut elves = elves.to_vec();

    for round in 0..ROUNDS {
        let mut new_moves = HashMap::with_capacity(elves.len());
        for (elf_index, elf) in elves.iter().enumerate() {
            if let Some(new_elf) = elf.tick(&a, round) {
                new_moves
                    .entry(new_elf)
                    .and_modify(|x| *x = None)
                    .or_insert(Some(elf_index));
            }
        }

        if new_moves.is_empty() {
            return ret(round, &elves);
        }

        for (new_pos, idx) in new_moves {
            if let Some(idx) = idx {
                a[elves[idx].0][elves[idx].1] = EMPTY;
                elves[idx] = new_pos;
                a[elves[idx].0][elves[idx].1] = ELF;
            }
        }
    }

    ret(ROUNDS, &elves)
}

#[aoc(day23, part1)]
pub fn part1((map, elves): &([[u8; WIDTH]; WIDTH], Vec<Elf>)) -> usize {
    solve::<10, _>(map, elves, |_, a| {
        let (y_min, y_max) = a.iter().map(|Elf(y, _)| *y).minmax().into_option().unwrap();
        let (x_min, x_max) = a.iter().map(|Elf(_, x)| *x).minmax().into_option().unwrap();

        (y_max.abs_diff(y_min) + 1) * (x_max.abs_diff(x_min) + 1) - a.len()
    })
}

#[aoc(day23, part2)]
pub fn part2((map, elves): &([[u8; WIDTH]; WIDTH], Vec<Elf>)) -> usize {
    solve::<1024, _>(map, elves, |round, _| round + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    const SAMPLE_SMALL: &str = r".....
..##.
..#..
.....
..##.
.....";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE_SMALL)), 30 - 5);
        assert_eq!(part1(&generator(SAMPLE)), 110);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 20);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day23.txt");
        const ANSWERS: (usize, usize) = (3864, 946);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
