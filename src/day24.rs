use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::astar;

use crate::common::utils::neighbors_and_self;

const NONE: u8 = 0;
const UP: u8 = 0b0001;
const DOWN: u8 = 0b0010;
const LEFT: u8 = 0b0100;
const RIGHT: u8 = 0b1000;
const WALL: u8 = 0b10000;

fn square2value(sq: u8) -> Option<u8> {
    Some(match sq {
        b'.' => NONE,
        b'^' => UP,
        b'v' => DOWN,
        b'<' => LEFT,
        b'>' => RIGHT,
        b'#' => WALL,
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
        16 => '#',
        _ => LOOKUP[x as usize] as char,
    }
}

fn wrap_math(pos: usize, max: usize, positive: bool) -> usize {
    match positive {
        true if pos < max - 2 => pos + 1,
        true => 1,
        false if pos == 1 => max - 2,
        false => pos - 1,
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldState {
    world: Vec<Vec<u8>>,
}

impl Debug for WorldState {
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

impl WorldState {
    fn tick(&self) -> Self {
        let height = self.world.len();
        let width = self.world[0].len();

        let mut world = vec![vec![NONE; width]; height];

        for (ridx, row) in self.world.iter().enumerate() {
            for (cidx, cell) in row.iter().enumerate() {
                if cell == &NONE {
                    continue;
                }

                if cell == &WALL {
                    world[ridx][cidx] = WALL;
                    continue;
                }

                if cell & UP == UP {
                    world[wrap_math(ridx, height, false)][cidx] |= UP;
                }

                if cell & DOWN == DOWN {
                    world[wrap_math(ridx, height, true)][cidx] |= DOWN;
                }

                if cell & LEFT == LEFT {
                    world[ridx][wrap_math(cidx, width, false)] |= LEFT;
                }

                if cell & RIGHT == RIGHT {
                    world[ridx][wrap_math(cidx, width, true)] |= RIGHT;
                }
            }
        }

        WorldState { world }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct World {
    states: Vec<WorldState>,
    height: usize,
    width: usize,
}

impl World {
    fn get(&mut self, n: usize) -> &WorldState {
        let l = self.states.len();
        for _ in l..=n {
            let new_state = self.states[self.states.len() - 1].tick();
            self.states.push(new_state);
        }

        self.states.get(n).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct State {
    pos: (usize, usize),
    world_num: usize,
}

#[aoc_generator(day24)]
pub fn generator(input: &str) -> World {
    let mut res = Vec::<Vec<u8>>::new();

    for line in input.lines() {
        if !line.is_empty() {
            res.push(line.bytes().filter_map(square2value).collect())
        }
    }
    let height = res.len();
    let width = res[0].len();

    World {
        states: vec![WorldState { world: res }],
        height,
        width,
    }
}

fn solve(
    worlds: &mut World,
    world_num: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    let height = worlds.height;
    let width = worlds.width;
    let res = astar(
        &State {
            pos: start,
            world_num,
        },
        |state| {
            let world_num = state.world_num + 1;
            let (r, c) = state.pos;
            neighbors_and_self(r, c, height, width)
                .filter(|&(y, x)| worlds.get(world_num).world[y][x] == NONE)
                .map(|pos| (State { pos, world_num }, 1))
                .collect::<Vec<_>>()
        },
        |state| state.pos.0.abs_diff(end.0) + state.pos.1.abs_diff(end.1),
        |state| state.pos == (end.0, end.1),
    );

    world_num + res.unwrap().1
}

#[aoc(day24, part1)]
pub fn part1(world: &World) -> usize {
    let mut world = world.clone();
    let start = (0, 1);
    let end = (world.height - 1, world.width - 2);

    solve(&mut world, 0, start, end)
}

#[aoc(day24, part2)]
pub fn part2(world: &World) -> usize {
    let mut world = world.clone();
    let start = (0, 1);
    let end = (world.height - 1, world.width - 2);

    let move_num = solve(&mut world, 0, start, end);
    let move_num = solve(&mut world, move_num, end, start);
    solve(&mut world, move_num, start, end)
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
        // println!("{:?}", generator(SAMPLE));

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
