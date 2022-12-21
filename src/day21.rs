use ahash::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Value(i64),
    Add(String, String),
    Mul(String, String),
    Div(String, String),
    Sub(String, String),
}

fn parse_line(s: &str) -> (String, Operation) {
    let (name, op) = s.split_once(": ").unwrap();

    let operation = if op.as_bytes()[0].is_ascii_digit() {
        Operation::Value(op.parse::<i64>().unwrap())
    } else {
        let l = op.split_once(&['*', '+', '/', '-'][..]).unwrap();
        let x = l.0.trim().to_string();
        let y = l.1.trim().to_string();

        match op.as_bytes()[5] {
            b'*' => Operation::Mul(x, y),
            b'+' => Operation::Add(x, y),
            b'/' => Operation::Div(x, y),
            b'-' => Operation::Sub(x, y),
            _ => panic!("Unknown operation"),
        }
    };

    (name.to_string(), operation)
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
    a: &'a str,
    b: &'a str,
    data: &HashMap<String, Operation>,
) -> Result<(i64, &'a str), (i64, &'a str)> {
    get(a, data)
        .map(|x| (x, b))
        .ok_or_else(|| get(b, data).map(|x| (x, a)).unwrap())
}

fn solve_p2(name: &str, data: &HashMap<String, Operation>, target: i64) -> i64 {
    if name == "humn" {
        target
    } else {
        let x = match &data[name] {
            Operation::Value(_) => todo!(),
            Operation::Add(a, b) => get_either(a, b, data),
            Operation::Mul(a, b) => get_either(a, b, data),
            Operation::Div(a, b) => get_either(a, b, data),
            Operation::Sub(a, b) => get_either(a, b, data),
        };

        match x {
            Ok((x, b)) => {
                let new_target = match &data[name] {
                    Operation::Value(_) => todo!(),
                    Operation::Add(_, _) => target - x,
                    Operation::Mul(_, _) => target / x,
                    Operation::Div(_, _) => x * target,
                    Operation::Sub(_, _) => x - target,
                };
                solve_p2(b, data, new_target)
            }
            Err((x, a)) => {
                let new_target = match &data[name] {
                    Operation::Value(_) => todo!(),
                    Operation::Add(_, _) => target - x,
                    Operation::Mul(_, _) => target / x,
                    Operation::Div(_, _) => target * x,
                    Operation::Sub(_, _) => target + x,
                };
                solve_p2(a, data, new_target)
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
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 152);
    }

    #[test]
    pub fn test2() {
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
