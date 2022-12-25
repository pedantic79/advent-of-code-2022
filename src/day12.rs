use crate::common::{pathfinding::bfs_count_bitset, utils::neighbors};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct HeightMap {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> HeightMap {
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

    HeightMap { map, start, end }
}

#[aoc(day12, part1)]
pub fn part1(inputs: &HeightMap) -> usize {
    let r_max = inputs.map.len();
    let c_max = inputs.map[0].len();

    bfs_count_bitset(
        &inputs.start,
        |state| {
            let height = inputs.map[state.0][state.1];

            neighbors(state.0, state.1, r_max, c_max)
                .filter(move |(y, x)| height + 1 >= inputs.map[*y][*x])
        },
        |state| state == &inputs.end,
        |x| x.0 * c_max + x.1,
    )
    .unwrap()
}

#[aoc(day12, part2)]
pub fn part2(inputs: &HeightMap) -> usize {
    let r_max = inputs.map.len();
    let c_max = inputs.map[0].len();
    bfs_count_bitset(
        &inputs.end,
        |state| {
            let height = inputs.map[state.0][state.1];

            neighbors(state.0, state.1, r_max, c_max)
                .filter(move |(y, x)| inputs.map[*y][*x] + 1 >= height)
        },
        |state| inputs.map[state.0][state.1] == b'a',
        |x| x.0 * c_max + x.1,
    )
    .unwrap()
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
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 31);
    }

    #[test]
    pub fn part2_test() {
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
