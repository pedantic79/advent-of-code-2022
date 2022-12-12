use crate::common::utils::neighbors;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::bfs;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Object {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.bytes()
                .enumerate()
                .map(|(c, cell)| match cell {
                    b'S' => {
                        start = (r, c);
                        b'a'
                    }
                    b'E' => {
                        end = (r, c);
                        b'z'
                    }
                    _ => cell,
                })
                .collect()
        })
        .collect();

    Object { map, start, end }
}

#[aoc(day12, part1)]
pub fn part1(inputs: &Object) -> usize {
    let r_max = inputs.map.len();
    let c_max = inputs.map[0].len();
    bfs(
        &inputs.start,
        |state| {
            let height = inputs.map[state.0][state.1];

            neighbors(state.0, state.1, r_max, c_max)
                .filter(|(y, x)| height + 1 >= inputs.map[*y][*x])
                .collect::<Vec<_>>()
        },
        |state| state == &inputs.end,
    )
    .unwrap()
    .len()
        - 1
}

#[aoc(day12, part2)]
pub fn part2(inputs: &Object) -> usize {
    let r_max = inputs.map.len();
    let c_max = inputs.map[0].len();
    (0..r_max)
        .cartesian_product(0..c_max)
        .filter(|(r, c)| inputs.map[*r][*c] == b'a')
        .map(|(r, c)| {
            bfs(
                &(r, c),
                |state| {
                    let height = inputs.map[state.0][state.1];

                    neighbors(state.0, state.1, r_max, c_max)
                        .filter(|(y, x)| height + 1 >= inputs.map[*y][*x])
                        .collect::<Vec<_>>()
                },
                |state| state == &inputs.end,
            )
            .map(|v| v.len())
            .unwrap_or(usize::MAX)
        })
        .min()
        .unwrap()
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 31);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 29);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day12.txt");
        const ANSWERS: (usize, usize) = (423, 416);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
