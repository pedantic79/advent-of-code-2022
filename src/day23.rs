use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Elf(i32, i32);

impl Elf {
    fn tick(&self, map: &HashSet<Elf>, i: usize) -> Option<Elf> {
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
                Some(Elf(self.0 + delta.0, self.1 + delta.1))
            } else {
                None
            }
        })
    }

    fn check(&self, map: &HashSet<Elf>, dirs: &[(i32, i32)]) -> bool {
        dirs.iter()
            .map(|(y, x)| Elf(self.0 + y, self.1 + x))
            .all(|elf| !map.contains(&elf))
    }
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> HashSet<Elf> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r_idx, row)| {
            row.bytes().enumerate().filter_map(move |(c_idx, cell)| {
                if cell == b'#' {
                    Some(Elf(r_idx as i32, c_idx as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn solve<const ROUNDS: usize, N>(
    inputs: &HashSet<Elf>,
    ret: impl Fn(usize, &HashSet<Elf>) -> N,
) -> N {
    let mut a = inputs.clone();
    let mut new_moves = HashMap::default();

    for round in 0..ROUNDS {
        let mut count = 0;
        for elf in a.iter() {
            if let Some(new_elf) = elf.tick(&a, round) {
                new_moves
                    .entry(new_elf)
                    .or_insert_with(|| Vec::with_capacity(4))
                    .push(*elf);
                count += 1;
            }
        }

        if count == 0 {
            return ret(round, &a);
        }

        for (new_pos, old_pos) in new_moves.iter_mut() {
            if old_pos.len() == 1 {
                a.remove(old_pos.first().unwrap());
                a.insert(*new_pos);
            }

            old_pos.clear();
        }
    }

    ret(ROUNDS, &a)
}

#[aoc(day23, part1)]
pub fn part1(inputs: &HashSet<Elf>) -> u32 {
    solve::<10, _>(inputs, |_, a| {
        let (y_min, y_max) = a.iter().map(|Elf(y, _)| *y).minmax().into_option().unwrap();
        let (x_min, x_max) = a.iter().map(|Elf(_, x)| *x).minmax().into_option().unwrap();

        (y_max.abs_diff(y_min) + 1) * (x_max.abs_diff(x_min) + 1) - (a.len() as u32)
    })
}

#[aoc(day23, part2)]
pub fn part2(inputs: &HashSet<Elf>) -> usize {
    solve::<1024, _>(inputs, |round, _| round + 1)
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
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE_SMALL)), 30 - 5);
        assert_eq!(part1(&generator(SAMPLE)), 110);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 20);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day23.txt");
        const ANSWERS: (u32, usize) = (3864, 946);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
