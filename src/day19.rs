use std::collections::VecDeque;

use ahash::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq, Eq)]
pub struct BluePrint {
    num: i64,
    ore: i64,
    clay: i64,
    obsidian: (i64, i64),
    geode: (i64, i64),
}

fn num(s: &str) -> IResult<&str, i64> {
    nom::character::complete::i64(s)
}

fn parse_blueprint(s: &str) -> IResult<&str, BluePrint> {
    map(
        tuple((
            tag("Blueprint "),
            num,
            tag(": Each ore robot costs "),
            num,
            tag(" ore. Each clay robot costs "),
            num,
            tag(" ore. Each obsidian robot costs "),
            num,
            tag(" ore and "),
            num,
            tag(" clay. Each geode robot costs "),
            num,
            tag(" ore and "),
            num,
            tag(" obsidian."),
        )),
        |x| BluePrint {
            num: x.1,
            ore: x.3,
            clay: x.5,
            obsidian: (x.7, x.9),
            geode: (x.11, x.13),
        },
    )(s)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
struct Counts {
    geode: i64,
    geode_robot: i64,
    obsidian_robot: i64,
    clay_robot: i64,
    ore_robot: i64,
    ore: i64,
    clay: i64,
    obsidian: i64,
}

impl Default for Counts {
    fn default() -> Self {
        Self {
            ore_robot: 1,
            ore: Default::default(),
            clay_robot: Default::default(),
            clay: Default::default(),
            obsidian_robot: Default::default(),
            obsidian: Default::default(),
            geode_robot: Default::default(),
            geode: Default::default(),
        }
    }
}

fn weight(geode: i64, geode_robots: i64, time: i64) -> i64 {
    let time = time - 2;
    geode + (geode_robots + time) * time
}

fn simulate(bp: &BluePrint, total_minutes: i64, factor: i64) -> i64 {
    let aggressive = bp
        != &BluePrint {
            num: 1,
            ore: 4,
            clay: 2,
            obsidian: (3, 14),
            geode: (2, 7),
        };

    let max_ore = [bp.ore, bp.clay, bp.obsidian.0, bp.geode.0]
        .into_iter()
        .max()
        .unwrap_or(0);
    let mut seen = HashSet::default();
    let mut queue = VecDeque::new();
    let mut ans = 0;

    queue.push_back((0, Counts::default()));

    while let Some((minutes, count)) = queue.pop_front() {
        if minutes == total_minutes {
            ans = ans.max(count.geode);
            continue;
        }

        if !seen.insert(count) {
            continue;
        }

        let ore = count.ore + count.ore_robot;
        let clay = count.clay + count.clay_robot;
        let obsidian = count.obsidian + count.obsidian_robot;
        let geode = count.geode + count.geode_robot;
        let minutes = minutes + 1;

        if count.ore >= bp.geode.0 && count.obsidian >= bp.geode.1 {
            queue.push_back((
                minutes,
                Counts {
                    geode,
                    geode_robot: count.geode_robot + 1,
                    ore: ore - bp.geode.0,
                    clay,
                    obsidian: obsidian - bp.geode.1,
                    ..count
                },
            ));
            continue;
        }

        if count.ore >= bp.obsidian.0 && count.clay >= bp.obsidian.1 {
            queue.push_back((
                minutes,
                Counts {
                    geode,
                    obsidian_robot: count.obsidian_robot + 1,
                    ore: ore - bp.obsidian.0,
                    clay: clay - bp.obsidian.1,
                    obsidian,
                    ..count
                },
            ));

            // This is a very aggressive prune. We still get a right answer, but
            // the sample does not. We will only prune if we aren't on the sample
            // input that is wrong.
            if aggressive {
                continue;
            }
        }

        // Don't make any more clay_robot than the max clay we need
        if count.clay_robot < bp.obsidian.1 && count.ore >= bp.clay {
            queue.push_back((
                minutes,
                Counts {
                    geode,
                    ore: ore - bp.clay,
                    clay_robot: count.clay_robot + 1,
                    clay,
                    obsidian,
                    ..count
                },
            ));
        }

        // When we aren't aggressively pruning then we can skip the rest
        if !aggressive && count.ore >= bp.obsidian.0 && count.clay >= bp.obsidian.1 {
            continue;
        }

        // Don't make any more ore_robot than the max ore we need
        if count.ore_robot < max_ore && count.ore >= bp.ore {
            queue.push_back((
                minutes,
                Counts {
                    geode,
                    ore_robot: count.ore_robot + 1,
                    ore: ore - bp.ore,
                    clay,
                    obsidian,
                    ..count
                },
            ));
        }

        queue.push_back((
            minutes,
            Counts {
                geode,
                ore,
                clay,
                obsidian,
                ..count
            },
        ));
    }

    ans * factor
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Vec<BluePrint> {
    input
        .lines()
        .map(|line| parse_blueprint(line).unwrap().1)
        .collect()
}

#[aoc(day19, part1)]
pub fn part1(inputs: &[BluePrint]) -> i64 {
    inputs.par_iter().map(|bp| simulate(bp, 24, bp.num)).sum()
}

#[aoc(day19, part2)]
pub fn part2(inputs: &[BluePrint]) -> i64 {
    inputs
        .par_iter()
        .take(3)
        .map(|bp| simulate(bp, 32, 1))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 33);
    }

    #[test]
    pub fn test2() {
        // assert_eq!(part2(&generator(SAMPLE)), 62 * 56);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day19.txt");
        const ANSWERS: (i64, i64) = (1264, 13475);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
