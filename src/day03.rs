use ahash::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

fn priority(b: u8) -> usize {
    match b {
        b'a'..=b'z' => (b - b'a' + 1) as usize,
        b'A'..=b'Z' => (b - b'A' + 26 + 1) as usize,
        _ => panic!("{b} isn't ascii"),
    }
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(inputs: &[String]) -> usize {
    inputs
        .iter()
        .map(|s| {
            let l = s.len() / 2;
            let mut freq: HashSet<u8> = HashSet::default();
            for s in s.bytes().take(l) {
                freq.insert(s);
            }

            for s in s.bytes().skip(l) {
                if freq.contains(&s) {
                    return priority(s);
                }
            }

            unreachable!()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(inputs: &[String]) -> usize {
    inputs
        .chunks(3)
        .map(|group| {
            let a = group[0].bytes().collect::<HashSet<u8>>();
            let b = group[1].bytes().collect::<HashSet<u8>>();
            let c = group[2].bytes().collect::<HashSet<u8>>();
            let ab = a.intersection(&b).copied().collect::<HashSet<u8>>();
            let z = ab.intersection(&c).copied().collect::<Vec<u8>>();
            priority(z[0])
        })
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
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 157);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 70);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day3.txt");
        const ANSWERS: (usize, usize) = (8153, 2342);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
