use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::Peekable;

fn process<'a, I: Iterator<Item = &'a str>>(
    lines: &mut Peekable<I>,
    sizes: &mut Vec<usize>,
) -> usize {
    let mut total = 0;

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            match line.rsplit_once(' ').unwrap().1 {
                ".." => break,
                "/" => continue,
                _ => total += process(lines, sizes),
            }
        } else if line == "$ ls" {
            while let Some(false) = lines.peek().map(|l| l.starts_with('$')) {
                let ls_output = lines.next().unwrap();
                if !ls_output.starts_with("dir") {
                    total += ls_output
                        .split_once(' ')
                        .unwrap()
                        .0
                        .parse::<usize>()
                        .unwrap();
                }
            }
        }
    }

    sizes.push(total);
    total
}

#[aoc_generator(day7)]
pub fn generator(inputs: &str) -> Vec<usize> {
    let mut res = Vec::new();
    process(&mut inputs.lines().peekable(), &mut res);
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
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
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
