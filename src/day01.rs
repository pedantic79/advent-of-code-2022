use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::common::nom::{nom_u32, process_input, separated_fold0};

#[aoc_generator(day01)]
pub fn generator(input: &str) -> Vec<u32> {
    process_input(separated_list1(
        tag("\n\n"),
        separated_fold0(tag("\n"), nom_u32, || 0, |acc: u32, n| acc + n),
    ))(input)
}

#[aoc(day01, part1, heap)]
pub fn part1(inputs: &[u32]) -> u32 {
    solve::<1>(inputs)
}

#[aoc(day01, part2, heap)]
pub fn part2(inputs: &[u32]) -> u32 {
    solve::<3>(inputs)
}

#[inline]
fn solve<const N: usize>(inputs: &[u32]) -> u32 {
    inputs
        .iter()
        .copied()
        .fold([0; N], crate::common::heap_retain::accumulate_max_n)
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        assert_eq!(generator(SAMPLE), &[6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 24000);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 45000);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day1.txt");
        const ANSWERS: (u32, u32) = (74198, 209914);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
