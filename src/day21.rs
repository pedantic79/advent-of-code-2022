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

#[aoc_generator(day21)]
pub fn generator(input: &str) -> HashMap<String, Operation> {
    input.lines().map(parse_line).collect()
}

#[aoc(day21, part1)]
pub fn part1(inputs: &HashMap<String, Operation>) -> i64 {
    solve("root", inputs)
}

#[aoc(day21, part2)]
pub fn part2(inputs: &HashMap<String, Operation>) -> i64 {
    let (a, b) = match &inputs["root"] {
        Operation::Add(a, b) => (a, b),
        Operation::Mul(a, b) => (a, b),
        Operation::Div(a, b) => (a, b),
        Operation::Sub(a, b) => (a, b),
        _ => panic!("bad type"),
    };

    let mut human = inputs
        .iter()
        .map(|(x, y)| (x.to_owned(), y.to_owned()))
        .collect::<HashMap<_, _>>();

    let mut lower = 100;
    let mut upper = 10_000_000_000_000;
    let mut last = 0;

    'outer: loop {
        let step = (upper - lower) / 1000;
        println!("{lower}..{upper}.step_by({step})");
        for x in (lower..upper).step_by(step as usize) {
            human.insert("humn".to_string(), Operation::Value(x));
            let x1 = solve(a, &human);
            let x2 = solve(b, &human);

            if x1 == x2 {
                break 'outer;
            } else if x2 - x1 > 0 && last < 0 {
                lower = x - step;
                upper = x;
                break;
            } else {
                last = x2 - x1;
            }
        }
    }

    for x in lower..upper {
        human.insert("humn".to_string(), Operation::Value(x));
        let x1 = solve(a, &human);
        let x2 = solve(b, &human);
        if x1 == x2 {
            return x;
        }
    }

    todo!()
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
        // assert_eq!(part2(&generator(SAMPLE)), 301);
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
