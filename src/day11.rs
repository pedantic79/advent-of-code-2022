use crate::common::heap_retain;
use aoc_runner_derive::{aoc, aoc_generator};
use std::{convert::Infallible, fmt::Debug, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    op: Op,
    test_divisor: u64,
    test_true: usize,
    test_false: usize,
}

fn parse_trailing_number<T>(s: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.rsplit_once(' ').unwrap().1.parse().unwrap()
}

fn parse_v<T, C>(s: &str) -> C
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    C: FromIterator<T>,
{
    let (_, x) = s.rsplit_once(':').unwrap();
    x.split(',')
        .map(|x| x.trim_start().parse().unwrap())
        .collect()
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();
        let items = parse_v(lines.next().unwrap());
        let op = lines.next().unwrap().parse().unwrap();
        let test_divisor = parse_trailing_number(lines.next().unwrap());
        let test_true = parse_trailing_number(lines.next().unwrap());
        let test_false = parse_trailing_number(lines.next().unwrap());

        Ok(Monkey {
            items,
            op,
            test_divisor,
            test_true,
            test_false,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

impl FromStr for Op {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, operation) = s.rsplit_once(" = ").unwrap();
        Ok(if operation.as_bytes()[6] == b'o' {
            Op::Square
        } else {
            let n = parse_trailing_number(operation);
            if operation.as_bytes()[4] == b'*' {
                Op::Mul(n)
            } else {
                Op::Add(n)
            }
        })
    }
}

impl Op {
    fn run(&self, other: u64) -> u64 {
        match self {
            Op::Add(x) => x + other,
            Op::Mul(x) => x * other,
            Op::Square => other * other,
        }
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|chunk| chunk.parse().unwrap())
        .collect()
}

fn solve<const ITERATIONS: usize>(
    monkeys: &[Monkey],
    worry_maintainer: impl Fn(u64) -> u64,
) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut inspects = vec![0; monkeys.len()];

    for _ in 0..ITERATIONS {
        for i in 0..monkeys.len() {
            inspects[i] += monkeys[i].items.len();
            for j in 0..monkeys[i].items.len() {
                let item = monkeys[i].items[j];
                let worry = worry_maintainer(monkeys[i].op.run(item));
                let idx = if worry % monkeys[i].test_divisor == 0 {
                    monkeys[i].test_true
                } else {
                    monkeys[i].test_false
                };
                monkeys[idx].items.push(worry);
            }
            monkeys[i].items.clear();
        }
    }

    let [a, b] = inspects
        .into_iter()
        .fold([0; 2], heap_retain::accumulate_max_n);
    a * b
}

#[aoc(day11, part1)]
pub fn part1(monkeys: &[Monkey]) -> usize {
    solve::<20>(monkeys, |x| x / 3)
}

#[aoc(day11, part2)]
pub fn part2(monkeys: &[Monkey]) -> usize {
    let all_divisor: u64 = monkeys.iter().map(|m| m.test_divisor).product();

    solve::<10000>(
        monkeys,
        |x| if x > all_divisor { x % all_divisor } else { x },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 10605);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 2713310158);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day11.txt");
        const ANSWERS: (usize, usize) = (111210, 15447387620);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
