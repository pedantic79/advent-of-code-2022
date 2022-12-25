use ahash::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, one_of},
    combinator::map,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Value(i64),
    Add(String, String),
    Mul(String, String),
    Div(String, String),
    Sub(String, String),
}

impl Operation {
    fn get_arguments(&self) -> (&str, &str) {
        match self {
            Operation::Value(_) => todo!(),
            Operation::Add(l, r) => (l, r),
            Operation::Mul(l, r) => (l, r),
            Operation::Div(l, r) => (l, r),
            Operation::Sub(l, r) => (l, r),
        }
    }
}

enum Either<L, R> {
    Left(L),
    Right(R),
}

fn parse_name(s: &str) -> IResult<&str, String> {
    map(take(4_usize), |x: &str| x.to_string())(s)
}

fn parse_op(s: &str) -> IResult<&str, Operation> {
    alt((
        map(complete::i64, Operation::Value),
        map(
            tuple((parse_name, tag(" "), one_of("+*/-"), tag(" "), parse_name)),
            |(l, _, op, _, r)| match op {
                '*' => Operation::Mul(l, r),
                '+' => Operation::Add(l, r),
                '/' => Operation::Div(l, r),
                '-' => Operation::Sub(l, r),
                _ => panic!("Unknown operation"),
            },
        ),
    ))(s)
}

fn parse_line(s: &str) -> (String, Operation) {
    map(
        tuple((parse_name, tag(": "), parse_op)),
        |(name, _, operation)| (name, operation),
    )(s)
    .unwrap()
    .1
}

fn solve(name: &str, data: &HashMap<String, Operation>) -> i64 {
    match &data[name] {
        Operation::Value(x) => *x,
        Operation::Add(a, b) => solve(a, data) + solve(b, data),
        Operation::Mul(a, b) => solve(a, data) * solve(b, data),
        Operation::Div(a, b) => solve(a, data) / solve(b, data),
        Operation::Sub(a, b) => solve(a, data) - solve(b, data),
    }
}

fn get(name: &str, data: &HashMap<String, Operation>) -> Option<i64> {
    if name == "humn" {
        None
    } else {
        Some(match &data[name] {
            Operation::Value(x) => *x,
            Operation::Add(a, b) => get(a, data)? + get(b, data)?,
            Operation::Mul(a, b) => get(a, data)? * get(b, data)?,
            Operation::Div(a, b) => get(a, data)? / get(b, data)?,
            Operation::Sub(a, b) => get(a, data)? - get(b, data)?,
        })
    }
}

fn get_either<'a>(
    (l, r): (&'a str, &'a str),
    data: &HashMap<String, Operation>,
) -> Either<(i64, &'a str), (i64, &'a str)> {
    if let Some(v) = get(l, data).map(|x| (x, r)) {
        Either::Left(v)
    } else if let Some(v) = get(r, data).map(|x| (x, l)) {
        Either::Right(v)
    } else {
        panic!("both sides are none found")
    }
}

fn solve_p2(name: &str, data: &HashMap<String, Operation>, target: i64) -> i64 {
    if name == "humn" {
        target
    } else {
        match get_either(data[name].get_arguments(), data) {
            Either::Left((x, r)) => {
                let new_target = match &data[name] {
                    Operation::Value(_) => todo!(),
                    Operation::Add(_, _) => target - x,
                    Operation::Mul(_, _) => target / x,
                    Operation::Div(_, _) => target * x,
                    Operation::Sub(_, _) => x - target,
                };
                solve_p2(r, data, new_target)
            }
            Either::Right((x, l)) => {
                let new_target = match &data[name] {
                    Operation::Value(_) => todo!(),
                    Operation::Add(_, _) => target - x,
                    Operation::Mul(_, _) => target / x,
                    Operation::Div(_, _) => target * x,
                    Operation::Sub(_, _) => target + x,
                };
                solve_p2(l, data, new_target)
            }
        }
    }
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> HashMap<String, Operation> {
    input.lines().map(parse_line).collect()
}

#[aoc(day21, part1)]
pub fn part1(data: &HashMap<String, Operation>) -> i64 {
    solve("root", data)
}

#[aoc(day21, part2)]
pub fn part2(data: &HashMap<String, Operation>) -> i64 {
    let (a, b) = match &data["root"] {
        Operation::Add(a, b) => (a, b),
        Operation::Mul(a, b) => (a, b),
        Operation::Div(a, b) => (a, b),
        Operation::Sub(a, b) => (a, b),
        _ => panic!("invalid root"),
    };

    // let mut data = data.clone();
    // data.insert("humn".to_owned(), Operation::Value(6915836165295));
    // println!("\na: {}", solve(a, &data));
    // println!("b: {}", solve(b, &data));

    match (get(a, data), get(b, data)) {
        (Some(x), None) => solve_p2(b, data, x),
        (None, Some(y)) => solve_p2(a, data, y),
        _ => panic!("both operations have humn"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 152);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 301);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day21.txt");
        const ANSWERS: (i64, i64) = (324122188240430, 3412650897405);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
