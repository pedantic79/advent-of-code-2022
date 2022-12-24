use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::bfs;

use crate::common::utils::neighbors;

const NONE: u8 = 0;
const UP: u8 = 0b0001;
const DOWN: u8 = 0b0010;
const LEFT: u8 = 0b0100;
const RIGHT: u8 = 0b1000;

fn square2value(sq: u8) -> Option<u8> {
    Some(match sq {
        b'.' => NONE,
        b'^' => UP,
        b'v' => DOWN,
        b'<' => LEFT,
        b'>' => RIGHT,
        b'#' => return None,
        _ => panic!("unknown character"),
    })
}

fn value2square(val: u8) -> char {
    const LOOKUP: &[u8] = b"0123456789abcdef";
    let x = val.count_ones();
    match val {
        0 => '.',
        1 => '^',
        2 => 'v',
        4 => '<',
        8 => '>',
        _ => LOOKUP[x as usize] as char,
    }
}

fn wrap_math(pos: usize, max: usize, positive: bool) -> usize {
    match positive {
        true if pos < max - 1 => pos + 1,
        true => 0,
        false if pos == 0 => max - 1,
        false => pos - 1,
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct World {
    world: Vec<Vec<u8>>,
}

impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for line in self.world.iter() {
            for cell in line.iter() {
                write!(f, "{}", value2square(*cell))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl World {
    fn tick(&self) -> Self {
        let height = self.world.len();
        let width = self.world[0].len();

        let mut world = vec![vec![NONE; width]; height];

        for (ridx, row) in self.world.iter().enumerate() {
            for (cidx, cell) in row.iter().enumerate() {
                if cell == &NONE {
                    continue;
                }

                if cell & UP > 0 {
                    world[wrap_math(ridx, height, false)][cidx] |= UP;
                }

                if cell & DOWN > 0 {
                    world[wrap_math(ridx, height, true)][cidx] |= DOWN;
                }

                if cell & LEFT > 0 {
                    world[ridx][wrap_math(cidx, width, false)] |= LEFT;
                }

                if cell & RIGHT > 0 {
                    world[ridx][wrap_math(cidx, width, true)] |= RIGHT;
                }
            }
        }

        World { world }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct State {
    pos: Pos,
    world_num: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Pos {
    Start,
    Some((usize, usize)),
    End,
}

#[aoc_generator(day24)]
pub fn generator(input: &str) -> World {
    let mut lines = input.lines();
    lines.next();

    let mut res = Vec::new();

    for line in lines {
        if !line.contains("##") {
            res.push(line.bytes().filter_map(square2value).collect())
        }
    }

    World { world: res }
}

fn get_world(n: usize, worlds: &mut Vec<World>) -> &World {
    while n >= worlds.len() {
        let new_state = worlds[worlds.len() - 1].tick();
        worlds.push(new_state);
    }
    worlds.get(n).unwrap()
}

fn solve(
    worlds: &mut Vec<World>,
    world_num: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    let height = worlds[0].world.len();
    let width = worlds[0].world[0].len();

    let res = bfs(
        &State {
            pos: Pos::Start,
            world_num,
        },
        |state| {
            let world_num = state.world_num + 1;
            match state.pos {
                Pos::Some((r, c)) => {
                    if (r, c) == end {
                        return vec![State {
                            pos: Pos::End,
                            world_num,
                        }];
                    }
                    std::iter::once((r, c))
                        .chain(neighbors(r, c, height, width))
                        .filter(|&(y, x)| get_world(world_num, worlds).world[y][x] == NONE)
                        .map(|pos| State {
                            pos: Pos::Some(pos),
                            world_num,
                        })
                        .collect()
                }
                Pos::Start => {
                    let mut v = vec![State {
                        pos: state.pos,
                        world_num,
                    }];

                    if get_world(world_num, worlds).world[start.0][start.1] == NONE {
                        v.push(State {
                            pos: Pos::Some((start.0, start.1)),
                            world_num,
                        })
                    }

                    v
                }
                Pos::End => vec![],
            }
        },
        |state| state.pos == Pos::End,
    );

    res.unwrap().len() - 1
}

#[aoc(day24, part1)]
pub fn part1(inputs: &World) -> usize {
    let height = inputs.world.len();
    let width = inputs.world[0].len();
    let mut worlds = vec![inputs.clone()];
    solve(&mut worlds, 0, (0, 0), (height - 1, width - 1))
}

#[aoc(day24, part2)]
pub fn part2(inputs: &World) -> usize {
    let height = inputs.world.len();
    let width = inputs.world[0].len();
    let mut worlds = vec![inputs.clone()];
    let mut move_num = solve(&mut worlds, 0, (0, 0), (height - 1, width - 1));
    move_num += solve(&mut worlds, move_num, (height - 1, width - 1), (0, 0));
    move_num + solve(&mut worlds, move_num, (0, 0), (height - 1, width - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 18);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 54);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day24.txt");
        const ANSWERS: (usize, usize) = (271, 813);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
