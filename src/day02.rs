use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    sequence::tuple,
};

use crate::common::nom::{nom_lines, process_input};

#[aoc_generator(day2)]
pub fn generator(input: &[u8]) -> Vec<(usize, usize)> {
    process_input(nom_lines(map(
        tuple((
            map(take::<_, _, ()>(1usize), |x: &[u8]| {
                usize::from(x[0] - b'A') + 1
            }),
            tag(" "),
            map(take(1usize), |x: &[u8]| usize::from(x[0] - b'X') + 1),
        )),
        |(l, _, r)| (l, r),
    )))(input)
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[(usize, usize)]) -> usize {
    inputs
        .iter()
        .map(|&(you, me)| me + 3 * ((4 + me - you) % 3))
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(inputs: &[(usize, usize)]) -> usize {
    inputs
        .iter()
        .map(|&(you, outcome)| ((you + outcome) % 3 + 1) + 3 * (outcome - 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = br"A Y
B X
C Z";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));
        // use RPS::*;
        // assert_eq!(
        //     generator(SAMPLE),
        //     &[(Rock, Paper), (Paper, Rock), (Scissors, Scissors)]
        // );
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 15);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 12);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day2.txt");
        const ANSWERS: (usize, usize) = (13682, 12881);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n').as_bytes();
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
