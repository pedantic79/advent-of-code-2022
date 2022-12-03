use aoc_runner_derive::{aoc, aoc_generator};

fn priority(b: u8) -> u64 {
    match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 26 + 1,
        _ => panic!("{b} isn't ascii"),
    }
    .into()
}

fn build_set(s: &str) -> u64 {
    s.bytes().fold(0, |set, c| set | 1 << priority(c))
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(inputs: &[String]) -> u64 {
    inputs
        .iter()
        .map(|s| {
            let (a, b) = s.split_at(s.len() / 2);

            let seen = build_set(a);
            b.bytes()
                .find_map(|c| {
                    let c = priority(c);
                    if seen & (1 << c) > 0 {
                        Some(c)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(inputs: &[String]) -> u64 {
    inputs
        .chunks(3)
        .map(|group| {
            u64::from(
                group
                    .iter()
                    .map(|g| build_set(g))
                    .reduce(|acc, set| acc & set)
                    .unwrap()
                    .trailing_zeros(),
            )
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
