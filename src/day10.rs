use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, sequence::preceded, IResult, Parser,
};

use crate::common::nom::{nom_i64, nom_lines, process_input};

#[derive(Debug, PartialEq, Eq)]
pub enum Instructions {
    Noop,
    Addx(i64),
}

fn parse_instructions(s: &str) -> IResult<&str, Instructions> {
    alt((
        map(tag("noop"), |_| Instructions::Noop),
        map(preceded(tag("addx "), nom_i64), Instructions::Addx),
    ))
    .parse(s)
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Instructions> {
    process_input(nom_lines(parse_instructions))(input)
}

fn solve(inputs: &[Instructions], mut update: impl FnMut(i64, i64)) {
    let mut x = 1;
    let mut cycle = 1;
    for ins in inputs {
        update(cycle, x);
        match ins {
            Instructions::Noop => {
                cycle += 1;
            }
            Instructions::Addx(n) => {
                cycle += 1;
                update(cycle, x);
                x += n;
                cycle += 1;
            }
        }
    }
}

#[aoc(day10, part1)]
pub fn part1(inputs: &[Instructions]) -> i64 {
    let mut total = 0;
    solve(inputs, |cycle, x| {
        if cycle % 40 == 20 {
            total += cycle * x;
        }
    });

    total
}

fn draw(screen: &mut [[u8; 40]; 6], cycle: i64, x: i64) {
    let cycle = cycle - 1;
    let row = cycle / 40;
    let col = cycle % 40;
    if col.abs_diff(x) <= 1 {
        screen[row as usize][col as usize] = b'#';
    }
}

#[aoc(day10, part2)]
pub fn part2(inputs: &[Instructions]) -> String {
    let mut screen = [[b'.'; 40]; 6];

    solve(inputs, |cycle, x| draw(&mut screen, cycle, x));

    unsafe {
        String::from_utf8_unchecked(screen.into_iter().fold(
            Vec::with_capacity(41 * 6),
            |mut acc, line| {
                acc.push(b'\n');
                acc.extend(line);
                acc
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("day10/SAMPLE2.txt");

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE.trim_end_matches('\n')));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE.trim_end_matches('\n'))), 13140);
    }

    #[test]
    pub fn part2_test() {
        const ANS: &str = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;

        assert_eq!(part2(&generator(SAMPLE.trim_end_matches('\n'))), ANS);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day10.txt");
        const ANSWERS: (i64, &str) = (
            13680,
            r#"
###..####..##..###..#..#.###..####.###..
#..#....#.#..#.#..#.#.#..#..#.#....#..#.
#..#...#..#....#..#.##...#..#.###..###..
###...#...#.##.###..#.#..###..#....#..#.
#....#....#..#.#....#.#..#....#....#..#.
#....####..###.#....#..#.#....####.###.."#,
        );

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
