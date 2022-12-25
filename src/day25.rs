use aoc_runner_derive::{aoc, aoc_generator};

fn to_dec(n: u8) -> i64 {
    match n {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'-' => -1,
        b'=' => -2,
        _ => panic!("invalid byte"),
    }
}

fn snafu2decimal(s: &[u8]) -> i64 {
    s.iter()
        .rev()
        .enumerate()
        .map(|(place, &b)| 5_i64.pow(place as u32) * to_dec(b))
        .sum()
}

#[aoc_generator(day25)]
pub fn generator(input: &str) -> i64 {
    input.lines().map(|l| snafu2decimal(l.as_bytes())).sum()
}

#[aoc(day25, part1)]
pub fn part1(original: &i64) -> String {
    let mut inputs = *original;
    let mut s = Vec::new();

    while inputs != 0 {
        let digit = inputs % 5;

        s.push(match digit {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => b'=',
            4 => b'-',
            _ => panic!("invalid math"),
        });

        inputs /= 5;
        inputs += digit / 3;
    }
    s.reverse();
    unsafe { String::from_utf8_unchecked(s) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));
        assert_eq!(generator(SAMPLE), 4890);

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn from_snafu() {
        const A: [(i64, &[u8]); 4] = [
            (2022, b"1=11-2"),
            (12345, b"1-0---0"),
            (314159265, b"1121-1110-1=0"),
            (27182818284, b"1-212-==002-112-"),
        ];

        for (i, s) in A {
            assert_eq!(snafu2decimal(s), i)
        }
    }

    #[test]
    pub fn count_snafu() {
        const A: &[&[u8]] = &[
            b"0", b"1", b"2", b"1=", b"1-", b"10", b"11", b"12", b"2=", b"2-", b"20", b"21", b"22",
            b"1==", b"1=-", b"1=0", b"1=1", b"1=2", b"1-=", b"1--", b"1-0", b"1-1", b"1-2", b"10=",
            b"10-", b"100", b"101", b"102", b"11=", b"11-", b"110", b"111", b"112", b"12=", b"12-",
            b"120", b"121", b"122", b"2==", b"2=-", b"2=0", b"2=1", b"2=2", b"2-=", b"2--", b"2-0",
        ];

        for (i, s) in A.iter().enumerate() {
            assert_eq!(snafu2decimal(s), i as i64)
        }
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), "2=-1=0");
        println!("{}", part1(&9890));
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day25.txt");
        const ANSWERS: &str = "20-=0=02=-21=00-02=2";

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(output, 37067035390042);
            assert_eq!(part1(&output), ANSWERS);
        }
    }
}
