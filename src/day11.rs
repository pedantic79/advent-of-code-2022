use crate::common::{
    heap_retain,
    nom::{nom_lines, nom_u64, nom_u8, nom_usize, process_input},
};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    op: Op,
    test_divisor: u64,
    test_true: usize,
    test_false: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Op {
    Add(u64),
    Mul(u64),
    Square,
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

fn parse_items(s: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(", "), nom_u64)(s)
}

fn parse_op(s: &str) -> IResult<&str, Op> {
    alt((
        map(tag("old * old"), |_| Op::Square),
        map(preceded(tag("old + "), nom_u64), Op::Add),
        map(preceded(tag("old * "), nom_u64), Op::Mul),
    ))(s)
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            tag("Monkey "),
            nom_u8,
            tag(":\n  Starting items: "),
            parse_items,
            tag("\n  Operation: new = "),
            parse_op,
            tag("\n  Test: divisible by "),
            nom_u64,
            tag("\n    If true: throw to monkey "),
            nom_usize,
            tag("\n    If false: throw to monkey "),
            nom_usize,
            opt(tag("\n")),
        )),
        |(_, _, _, items, _, op, _, test_divisor, _, test_true, _, test_false, _)| Monkey {
            items,
            op,
            test_divisor,
            test_true,
            test_false,
        },
    )(s)
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<Monkey> {
    process_input(nom_lines(parse_monkey))(input)
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
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 10605);
    }

    #[test]
    pub fn part2_test() {
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
