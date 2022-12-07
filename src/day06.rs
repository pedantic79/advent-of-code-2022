use aoc_runner_derive::aoc;

fn unique(s: &[u8]) -> Result<(), usize> {
    for i in (1..s.len()).rev() {
        for j in (0..i).rev() {
            if s[i] == s[j] {
                return Err(j + 1);
            }
        }
    }
    Ok(())
}

fn solve<const N: usize>(input: &str) -> usize {
    let input = input.as_bytes();
    let mut pos = 0;

    loop {
        assert!(pos + N < input.len());
        if let Err(x) = unique(&input[pos..(pos + N)]) {
            pos += x;
        } else {
            break pos + N;
        }
    }
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
