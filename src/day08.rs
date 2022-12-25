use crate::common::utils::neighbors;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn check_visible(inputs: &[Vec<u8>], r: usize, c: usize, h: usize, w: usize) -> bool {
    let height = inputs[r][c];

    inputs[r][..c].iter().all(|&hx| hx < height)
        || inputs[r][(c + 1)..w].iter().all(|&hx| hx < height)
        || (0..r).map(|hi| inputs[hi][c]).all(|hx| hx < height)
        || ((r + 1)..h).map(|hi| inputs[hi][c]).all(|hx| hx < height)
}

fn count_trees(i: impl Iterator<Item = u8>, height: u8) -> usize {
    let mut count = 0;
    for hx in i {
        count += 1;
        if hx >= height {
            break;
        }
    }
    count
}

fn calculate_score(inputs: &[Vec<u8>], r: usize, c: usize, h: usize, w: usize) -> usize {
    let height = inputs[r][c];

    count_trees((0..r).rev().map(|hi| inputs[hi][c]), height)
        * count_trees(inputs[r][..c].iter().rev().copied(), height)
        * count_trees(((r + 1)..h).map(|hi| inputs[hi][c]), height)
        * count_trees(inputs[r][(c + 1)..w].iter().copied(), height)
}

#[aoc(day8, part1)]
pub fn part1(inputs: &[Vec<u8>]) -> usize {
    let height = inputs.len();
    let width = inputs[0].len();
    let mut count = (width - 1) * 2 + (height - 1) * 2;

    for r in 0..height {
        for c in 0..height {
            if neighbors(r, c, height, width).count() == 4
                && check_visible(inputs, r, c, height, width)
            {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day8, part2)]
pub fn part2(inputs: &[Vec<u8>]) -> usize {
    let height = inputs.len();
    let width = inputs[0].len();
    let mut max = 0;

    for r in 0..height {
        for c in 0..height {
            let score = calculate_score(inputs, r, c, height, width);
            max = max.max(score);
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"30373
25512
65332
33549
35390";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 21);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 8);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day8.txt");
        const ANSWERS: (usize, usize) = (1794, 199272);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
