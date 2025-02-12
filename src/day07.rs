use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::alpha1,
    combinator::{map, opt},
    multi::fold_many0,
    sequence::{preceded, terminated},
    IResult, Parser,
};

use crate::common::nom::nom_usize;

fn filename(s: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == '.' || c.is_alphabetic())(s)
}

fn dirname(s: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == '/' || c == '.' || c.is_alphabetic())(s)
}

fn ls(s: &str) -> IResult<&str, usize> {
    preceded(
        tag("$ ls\n"),
        fold_many0(
            terminated(
                alt((
                    map((tag("dir "), alpha1), |_| 0),
                    map((nom_usize, tag(" "), filename), |(n, _, _)| n),
                )),
                opt(tag("\n")),
            ),
            || 0,
            |acc, n| acc + n,
        ),
    )
    .parse(s)
}

fn cd(s: &str) -> IResult<&str, &str> {
    map((tag("$ cd "), dirname, tag("\n")), |(_, name, _)| name).parse(s)
}

fn process(s: &mut &str, sizes: &mut Vec<usize>) -> usize {
    let mut total = 0;

    while !s.is_empty() {
        if let Ok((rem_s, name)) = cd(s) {
            *s = rem_s;
            match name {
                ".." => break,
                "/" => continue,
                _ => total += process(s, sizes),
            }
        } else if let Ok((rem_s, size)) = ls(s) {
            *s = rem_s;
            total += size;
        } else {
            panic!("unknown input: {s}")
        }
    }

    sizes.push(total);
    total
}

#[aoc_generator(day7)]
pub fn generator(mut inputs: &str) -> Vec<usize> {
    let mut res = Vec::new();
    process(&mut inputs, &mut res);
    res
}

#[aoc(day7, part1)]
pub fn part1(dirs: &[usize]) -> usize {
    dirs.iter().filter(|&&v| v < 100000).sum()
}

#[aoc(day7, part2)]
pub fn part2(dirs: &[usize]) -> usize {
    let need = 30000000 - (70000000 - dirs[dirs.len() - 1]);
    *dirs.iter().filter(|&&v| v > need).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn ls_test() {
        const A: &str = "$ ls
dir fcqv
dir fcv
72939 hdpgfcwd
236918 jlncjqh.csz
dir jvwfwrg
dir tzwpllhq
dir vglf
28586 wzljr.zvp";
        assert_eq!(ls(A).unwrap().1, 72939 + 236918 + 28586);
    }

    #[test]
    pub fn cd_test() {
        const A: &str = "$ cd /
";
        assert_eq!(cd(A).unwrap().1, "/");
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 95437);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 24933642);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day7.txt");
        const ANSWERS: (usize, usize) = (1182909, 2832508);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
