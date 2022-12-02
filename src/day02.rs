use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            (
                usize::from(a.as_bytes()[0] - b'A') + 1,
                usize::from(b.as_bytes()[0] - b'X') + 1,
            )
        })
        .collect()
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

    const SAMPLE: &str = r"A Y
B X
C Z";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));
        // use RPS::*;
        // assert_eq!(
        //     generator(SAMPLE),
        //     &[(Rock, Paper), (Paper, Rock), (Scissors, Scissors)]
        // );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 15);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 12);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day2.txt");
        const ANSWERS: (usize, usize) = (13682, 12881);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
