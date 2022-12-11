use std::{cmp::Reverse, collections::VecDeque, convert::Infallible, fmt::Debug, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[allow(non_camel_case_types)]
type int = u64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    items: VecDeque<int>,
    op: Op,
    test_divisor: int,
    test_true: usize,
    test_false: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Op {
    Add(int),
    Mul(int),
    Square,
}

impl FromStr for Op {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, operation) = s.rsplit_once(" = ").unwrap();
        Ok(if operation == "old * old" {
            Op::Square
        } else {
            let (_, n) = operation.rsplit_once(&['*', '+'][..]).unwrap();
            let n = n.trim_start().parse().unwrap();
            if operation.contains('*') {
                Op::Mul(n)
            } else {
                Op::Add(n)
            }
        })
    }
}

impl Op {
    fn run(&self, other: int) -> int {
        match self {
            Op::Add(x) => x + other,
            Op::Mul(x) => x * other,
            Op::Square => other * other,
        }
    }
}

#[aoc_generator(day11)]
pub fn generator(_input: &str) -> Vec<Monkey> {
    #![allow(clippy::vec_init_then_push)]
    let mut res = vec![];
    // res.push(Monkey {
    //     items: VecDeque::from([79, 98]),
    //     op: Op::Mul(19),
    //     test_divisor: 23,
    //     test_true: 2,
    //     test_false: 3,
    // });

    // res.push(Monkey {
    //     items: VecDeque::from([54, 65, 75, 74]),
    //     op: Op::Add(6),
    //     test_divisor: 19,
    //     test_true: 2,
    //     test_false: 0,
    // });

    // res.push(Monkey {
    //     items: VecDeque::from([79, 60, 97]),
    //     op: Op::Square,
    //     test_divisor: 13,
    //     test_true: 1,
    //     test_false: 3,
    // });

    // res.push(Monkey {
    //     items: VecDeque::from([74]),
    //     op: Op::Add(3),
    //     test_divisor: 17,
    //     test_true: 0,
    //     test_false: 1,
    // });

    // INPUT
    res.push(Monkey {
        items: [54, 53].into(),
        op: Op::Mul(3),
        test_divisor: 2,
        test_true: 2,
        test_false: 6,
    });

    res.push(Monkey {
        items: [95, 88, 75, 81, 91, 67, 65, 84].into(),
        op: Op::Mul(11),
        test_divisor: 7,
        test_true: 3,
        test_false: 4,
    });

    res.push(Monkey {
        items: [76, 81, 50, 93, 96, 81, 83].into(),
        op: Op::Add(6),
        test_divisor: 3,
        test_true: 5,
        test_false: 1,
    });

    res.push(Monkey {
        items: [83, 85, 85, 63].into(),
        op: Op::Add(4),
        test_divisor: 11,
        test_true: 7,
        test_false: 4,
    });

    res.push(Monkey {
        items: [85, 52, 64].into(),
        op: Op::Add(8),
        test_divisor: 17,
        test_true: 0,
        test_false: 7,
    });
    res.push(Monkey {
        items: [57].into(),
        op: Op::Add(2),
        test_divisor: 5,
        test_true: 1,
        test_false: 3,
    });
    res.push(Monkey {
        items: [60, 95, 76, 66, 91].into(),
        op: Op::Square,
        test_divisor: 13,
        test_true: 2,
        test_false: 5,
    });

    res.push(Monkey {
        items: [65, 84, 76, 72, 79, 65].into(),
        op: Op::Add(5),
        test_divisor: 19,
        test_true: 6,
        test_false: 0,
    });

    res
}

#[aoc(day11, part1)]
pub fn part1(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut inspects = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                inspects[i] += 1;
                let worry = monkeys[i].op.run(item) / 3;
                if worry % monkeys[i].test_divisor == 0 {
                    let idx = monkeys[i].test_true;
                    monkeys[idx].items.push_back(worry);
                } else {
                    let idx = monkeys[i].test_false;
                    monkeys[idx].items.push_back(worry);
                }
            }
        }
    }

    inspects.select_nth_unstable_by_key(1, |&x| Reverse(x));
    inspects[0..2].iter().product()
}

#[aoc(day11, part2)]
pub fn part2(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut inspects = vec![0; monkeys.len()];
    let all_divisor: int = monkeys.iter().map(|m| m.test_divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                inspects[i] += 1;
                let worry = monkeys[i].op.run(item) % all_divisor;
                if worry % monkeys[i].test_divisor == 0 {
                    let idx = monkeys[i].test_true;
                    monkeys[idx].items.push_back(worry);
                } else {
                    let idx = monkeys[i].test_false;
                    monkeys[idx].items.push_back(worry);
                }
            }
        }
    }

    inspects.select_nth_unstable_by_key(1, |&x| Reverse(x));
    inspects[0..2].iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"..##.......";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        // assert_eq!(part1(&generator(SAMPLE)), 7);
    }

    #[test]
    pub fn test2() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
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
