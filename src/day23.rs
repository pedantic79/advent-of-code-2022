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

fn apply_delta(pos: usize, delta: (i32, i32)) -> usize {
    (pos as i32 + delta.0 * WIDTH as i32 + delta.1) as usize
}

fn tick(elf: usize, map: &[u8], i: usize) -> Option<usize> {
    if check(elf, map, &ALL) {
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
        if check(elf, map, dir) {
            Some(apply_delta(elf, *delta))
        } else {
            None
        }
    })
}

fn check(elf: usize, map: &[u8], dirs: &[(i32, i32)]) -> bool {
    dirs.iter()
        .map(|delta| map[apply_delta(elf, *delta)])
        .all(|elf| elf == EMPTY)
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> ([u8; WIDTH * WIDTH], Vec<usize>) {
    let mut map = [EMPTY; WIDTH * WIDTH];
    let mut elves = Vec::new();
    for (r_idx, row) in input.lines().enumerate() {
        for (c_idx, cell) in row.bytes().enumerate() {
            if cell == ELF {
                let index = (r_idx + OFFSET) * WIDTH + (c_idx + OFFSET);
                map[index] = ELF;
                elves.push(index);
            }
        }
    }

    (map, elves)
}

fn solve<const ROUNDS: usize, N>(
    a: &[u8; WIDTH * WIDTH],
    elves: &[usize],
    ret: impl Fn(usize, &[usize]) -> N,
) -> N {
    let mut a = *a;
    let mut elves = elves.to_vec();

    for round in 0..ROUNDS {
        let mut new_moves = HashMap::with_capacity(elves.len());
        for (elf_index, elf) in elves.iter().enumerate() {
            if let Some(new_elf) = tick(*elf, &a, round) {
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
                a[elves[idx]] = EMPTY;
                elves[idx] = new_pos;
                a[elves[idx]] = ELF;
            }
        }
    }

    ret(ROUNDS, &elves)
}

#[aoc(day23, part1)]
pub fn part1((map, elves): &([u8; WIDTH * WIDTH], Vec<usize>)) -> usize {
    solve::<10, _>(map, elves, |_, a| {
        let (y_min, y_max) = a.iter().map(|y| y / WIDTH).minmax().into_option().unwrap();
        let (x_min, x_max) = a.iter().map(|x| x % WIDTH).minmax().into_option().unwrap();

        (y_max.abs_diff(y_min) + 1) * (x_max.abs_diff(x_min) + 1) - a.len()
    })
}

#[aoc(day23, part2)]
pub fn part2((map, elves): &([u8; WIDTH * WIDTH], Vec<usize>)) -> usize {
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
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

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
