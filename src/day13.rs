use aoc_runner_derive::{aoc, aoc_generator};
use nom::{branch::alt, bytes::complete::tag, multi::separated_list1, sequence::tuple, IResult};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Signal {
    List(Vec<Signal>),
    Value(u32),
}

fn num(s: &str) -> IResult<&str, Signal> {
    let (s, val) = nom::character::complete::u32(s)?;

    Ok((s, Signal::Value(val)))
}

fn signal(s: &str) -> IResult<&str, Signal> {
    alt((num, list, empty))(s)
}

fn empty(s: &str) -> IResult<&str, Signal> {
    let (s, _) = tuple((tag("["), tag("]")))(s)?;

    Ok((s, Signal::List(vec![])))
}

fn list(s: &str) -> IResult<&str, Signal> {
    let (s, (_, sig, _)) = tuple((tag("["), separated_list1(tag(","), signal), tag("]")))(s)?;

    Ok((s, Signal::List(sig)))
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Signal::Value(a), Signal::Value(b)) => a.cmp(b),
            (Signal::Value(a), Signal::List(b)) => vec![Signal::Value(*a)].cmp(b),
            (Signal::List(a), Signal::Value(b)) => a.cmp(&vec![Signal::Value(*b)]),
            (Signal::List(a), Signal::List(b)) => a.cmp(b),
        }
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|x| {
            let (a, b) = x.split_once('\n').unwrap();

            (signal(a).unwrap().1, signal(b).unwrap().1)
        })
        .enumerate()
        .filter_map(|(i, (a, b))| if a <= b { Some(i + 1) } else { None })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let a = signal("[[2]]").unwrap().1;
    let b = signal("[[6]]").unwrap().1;

    let mut v: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|x| signal(x).unwrap().1)
        .collect();

    v.push(a.clone());
    v.push(b.clone());
    v.sort_unstable();

    let a = (1..)
        .zip(v.iter())
        .find_map(|x| if x.1 == &a { Some(x.0) } else { None })
        .unwrap();

    let b = (1..)
        .zip(v.iter())
        .find_map(|x| if x.1 == &b { Some(x.0) } else { None })
        .unwrap();

    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator1(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(SAMPLE), 13);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(SAMPLE), 140);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day13.txt");
        const ANSWERS: (usize, usize) = (5852, 24190);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            // let output = generator(input);

            assert_eq!(part1(input), ANSWERS.0);
            assert_eq!(part2(input), ANSWERS.1);
        }
    }
}
