use aoc_runner_derive::aoc;

fn unique(s: &[u8]) -> bool {
    for i in 0..s.len() {
        for j in (i + 1)..s.len() {
            if s[i] == s[j] {
                return false;
            }
        }
    }

    true
}

fn solve<const N: usize>(input: &str) -> usize {
    input
        .as_bytes()
        .windows(N)
        .enumerate()
        .find_map(|(i, w)| if unique(w) { Some(i) } else { None })
        .unwrap()
        + N
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    solve::<4>(input)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    solve::<14>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        for (&input, ans) in SAMPLE.iter().zip([7, 5, 6, 10, 11]) {
            assert_eq!(part1(input), ans);
        }
    }

    #[test]
    pub fn test2() {
        for (&input, ans) in SAMPLE.iter().zip([19, 23, 23, 29, 26]) {
            assert_eq!(part2(input), ans);
        }
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day6.txt");
        const ANSWERS: (usize, usize) = (1100, 2421);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = input;

            assert_eq!(part1(output), ANSWERS.0);
            assert_eq!(part2(output), ANSWERS.1);
        }
    }
}
