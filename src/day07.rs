use aoc_runner_derive::{aoc, aoc_generator};
use nohash_hasher::IntMap;
use std::hash::Hasher;
use std::{
    hash::Hash,
    path::{Path, PathBuf},
};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = rustc_hash::FxHasher::default();
    t.hash(&mut s);
    s.finish()
}

#[aoc_generator(day7)]
pub fn generator(inputs: &str) -> Vec<usize> {
    let mut dirs: IntMap<u64, usize> = IntMap::default();
    let mut path = PathBuf::new();
    let mut iter = inputs.lines().peekable();
    let mut root = 0;

    while let Some(line) = iter.next() {
        if line.starts_with("$ cd") {
            let dir = line.rsplit_once(' ').unwrap().1;
            if dir != ".." {
                path.push(dir);
            } else {
                path.pop();
            }
        } else if line == "$ ls" {
            let mut total: usize = 0;
            while let Some(false) = iter.peek().map(|l| l.starts_with('$')) {
                let ls_output = iter.next().unwrap();
                if !ls_output.starts_with("dir") {
                    total += ls_output
                        .split_once(' ')
                        .unwrap()
                        .0
                        .parse::<usize>()
                        .unwrap();
                }
            }

            let mut d = path.clone();
            while d != Path::new("/") {
                *dirs.entry(calculate_hash(&d)).or_default() += total;
                d.pop();
            }
            root += total;
        }
    }

    let mut res = Vec::with_capacity(dirs.len() + 1);
    res.push(root);
    res.extend(dirs.into_values());
    res
}

#[aoc(day7, part1)]
pub fn part1(dirs: &[usize]) -> usize {
    dirs.iter().filter(|&&v| v < 100000).sum()
}

#[aoc(day7, part2)]
pub fn part2(dirs: &[usize]) -> usize {
    let need = 30000000 - (70000000 - dirs[0]);
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
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 95437);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 24933642);
    }

    #[test]
    fn hashing() {
        assert_eq!(
            calculate_hash(&["/"].iter().collect::<PathBuf>()),
            calculate_hash(&Path::new("/"))
        )
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
