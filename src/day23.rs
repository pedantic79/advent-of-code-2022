use ahash::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

const ALL: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const NORTH: [(i64, i64); 3] = [(-1, -1), (-1, 0), (-1, 1)];
const SOUTH: [(i64, i64); 3] = [(1, -1), (1, 0), (1, 1)];
const WEST: [(i64, i64); 3] = [(-1, -1), (0, -1), (1, -1)];
const EAST: [(i64, i64); 3] = [(-1, 1), (0, 1), (1, 1)];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Elf(i64, i64);

impl Elf {
    fn tick(&self, map: &[Elf], i: usize) -> Option<Elf> {
        if self.check(map, &ALL) {
            return None;
        }

        for (dir, delta) in [
            (NORTH, (-1, 0)),
            (SOUTH, (1, 0)),
            (WEST, (0, -1)),
            (EAST, (0, 1)),
        ]
        .iter()
        .cycle()
        .skip(i)
        .take(4)
        {
            if self.check(map, dir) {
                return Some(Elf(self.0 + delta.0, self.1 + delta.1));
            }
        }

        None
    }

    fn check(&self, map: &[Elf], dirs: &[(i64, i64)]) -> bool {
        dirs.iter()
            .map(|(y, x)| Elf(self.0 + y, self.1 + x))
            .all(|elf| !map.contains(&elf))
    }
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<Elf> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r_idx, row)| {
            row.bytes().enumerate().filter_map(move |(c_idx, cell)| {
                if cell == b'#' {
                    Some(Elf(r_idx as i64, c_idx as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[aoc(day23, part1)]
pub fn part1(inputs: &[Elf]) -> u64 {
    let mut a = inputs.to_vec();

    for round in 0..10 {
        let mut new_moves = HashMap::default();
        for (id, elf) in a.iter().enumerate() {
            if let Some(new_elf) = elf.tick(&a, round) {
                new_moves.entry(new_elf).or_insert_with(Vec::new).push(id);
            }
        }

        if new_moves.is_empty() {
            break;
        }

        for (pos, elves) in new_moves.iter() {
            if elves.len() == 1 {
                a[elves[0]] = *pos;
            }
        }
    }

    let (y_min, y_max) = a.iter().map(|Elf(y, _)| *y).minmax().into_option().unwrap();
    let (x_min, x_max) = a.iter().map(|Elf(_, x)| *x).minmax().into_option().unwrap();

    (y_max.abs_diff(y_min) + 1) * (x_max.abs_diff(x_min) + 1) - (a.len() as u64)
}

#[aoc(day23, part2)]
pub fn part2(inputs: &[Elf]) -> usize {
    let mut a = inputs.to_vec();

    for round in 0.. {
        let mut new_moves = HashMap::default();
        for (id, elf) in a.iter().enumerate() {
            if let Some(new_elf) = elf.tick(&a, round) {
                new_moves.entry(new_elf).or_insert_with(Vec::new).push(id);
            }
        }

        if new_moves.is_empty() {
            return round + 1;
        }

        for (pos, elves) in new_moves.iter() {
            if elves.len() == 1 {
                a[elves[0]] = *pos;
            }
        }
    }

    unimplemented!()
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
        const ANSWERS: (u64, usize) = (3864, 946);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
