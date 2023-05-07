use aoc_runner_derive::{aoc, aoc_generator};
use nom::{character::complete::alpha1, combinator::map};

use crate::common::nom::{nom_lines, process_input};

// Amount to subtract to
const OFFSET: u8 = 64;

fn calc_priority(set: u64) -> u64 {
    // convert the set bit in the set back to the character
    let c = u8::try_from(set.trailing_zeros()).unwrap() + OFFSET;

    if c.is_ascii_lowercase() {
        c - b'a' + 1
    } else {
        c - b'A' + 27
    }
    .into()
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<String> {
    process_input(nom_lines(map(alpha1::<_, ()>, |s: &str| s.to_string())))(input)
}

#[aoc(day3, part1)]
pub fn part1(rucksacks: &[String]) -> u64 {
    rucksacks
        .iter()
        .map(|rucksack| {
            let (l, r) = rucksack.split_at(rucksack.len() / 2);
            let (mut left, mut rite) = (0, 0);

            // loop until we have a matching bits
            for (left_byte, rite_byte) in l.bytes().zip(r.bytes()) {
                left |= 1 << (left_byte - OFFSET);
                rite |= 1 << (rite_byte - OFFSET);
                let cross = left & rite;
                if cross > 0 {
                    return calc_priority(cross);
                }
            }

            panic!("{l} and {r} don't overlap")
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(rucksacks: &[String]) -> u64 {
    rucksacks
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|rucksack| rucksack.bytes().fold(0, |set, c| set | 1 << (c - OFFSET)))
                .reduce(|acc, set| acc & set)
                .unwrap()
        })
        .map(calc_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 157);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 70);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day3.txt");
        const ANSWERS: (u64, u64) = (8153, 2342);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
