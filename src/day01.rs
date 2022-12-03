use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day01)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse::<usize>().unwrap()).sum())
        .collect()
}

#[aoc(day01, part1, heap)]
pub fn part1(inputs: &[usize]) -> usize {
    solve::<1>(inputs)
}

#[aoc(day01, part2, heap)]
pub fn part2(inputs: &[usize]) -> usize {
    solve::<3>(inputs)
}

#[inline]
fn solve<const N: usize>(inputs: &[usize]) -> usize {
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
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 24000);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 45000);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day1.txt");
        const ANSWERS: (usize, usize) = (74198, 209914);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
