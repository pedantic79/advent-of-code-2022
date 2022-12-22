use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{complete, map},
    multi::many0,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Moves {
    Forward(u32),
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn value(&self) -> usize {
        match self {
            Dir::Left => 2,
            Dir::Right => 0,
            Dir::Up => 3,
            Dir::Down => 1,
        }
    }

    fn from_value(n: usize) -> Self {
        match n % 4 {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => panic!("not %4"),
        }
    }

    fn forward_one(&self, (y, x): (usize, usize), y_max: usize, x_max: usize) -> (usize, usize) {
        match self {
            Dir::Left if x > 1 => (y, x - 1),
            Dir::Left => (y, x_max),
            Dir::Right if x < x_max => (y, x + 1),
            Dir::Right => (y, 0),
            Dir::Up if y > 1 => (y - 1, x),
            Dir::Up => (y_max, x),
            Dir::Down if y < y_max => (y + 1, x),
            Dir::Down => (0, x),
        }
    }

    fn forward_one_signed(&self, (y, x): (isize, isize)) -> (isize, isize) {
        match self {
            Dir::Left => (y, x - 1),
            Dir::Right => (y, x + 1),
            Dir::Up => (y - 1, x),
            Dir::Down => (y + 1, x),
        }
    }

    fn turn(&self, m: &Moves) -> Self {
        match m {
            Moves::Forward(_) => panic!("can't turn a Forward"),
            Moves::Right => Self::from_value((self.value() + 1) % 4),
            Moves::Left => Self::from_value((self.value() + 3) % 4),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    mv: Vec<Moves>,
    board: Vec<Vec<u8>>,
}

impl Input {
    fn find_start(&self) -> (usize, usize) {
        (0, self.board[0].iter().position(|&x| x == b'.').unwrap())
    }

    fn board_get(&self, pos: (usize, usize)) -> Option<u8> {
        let row = self.board.get(pos.0)?;
        let cell = row.get(pos.1)?;

        if *cell == b' ' {
            None
        } else {
            Some(*cell)
        }
    }

    fn part1_warp(&self, direction: &Dir, mut pos: (usize, usize)) -> (usize, usize) {
        loop {
            pos = direction.forward_one(pos, self.board.len(), self.board[0].len());
            if self.board_get(pos).is_some() {
                break pos;
            }
        }
    }

    fn move_forward1(&self, direction: Dir, n: u32, mut pos: (usize, usize)) -> (usize, usize) {
        for _ in 0..n {
            let new_pos = direction.forward_one(pos, self.board.len(), self.board[0].len());
            match self.board_get(new_pos) {
                Some(b'#') => {
                    break;
                }
                Some(_) => {
                    pos = new_pos;
                }
                None => {
                    pos = self.part1_warp(&direction, new_pos);
                    continue;
                }
            }
        }

        pos
    }

    fn move_forward2(
        &self,
        direction: &mut Dir,
        n: u32,
        mut pos: (isize, isize),
    ) -> (isize, isize) {
        for _ in 0..n {
            let new_pos = direction.forward_one_signed(pos);
            let mut new_direction = *direction;

            let new_pos = match direction {
                Dir::Up | Dir::Down => wrap_y(&mut new_direction, new_pos),
                Dir::Left | Dir::Right => wrap_x(&mut new_direction, new_pos),
            };

            match self.board_get(to_usize(new_pos)) {
                Some(b'#') | None => {}
                Some(_) => {
                    pos = new_pos;
                    *direction = new_direction;
                }
            }
        }

        pos
    }
}

fn wrap_y(direction: &mut Dir, (y, x): (isize, isize)) -> (isize, isize) {
    match x {
        0..=49 if y < 100 => {
            *direction = Dir::from_value(direction.value() + 1);
            (x + 50, 50)
        }
        0..=49 if y >= 200 => (0, x + 100),
        50..=99 if y < 0 => {
            *direction = Dir::from_value(direction.value() + 1);
            (x + 100, 0)
        }
        50..=99 if y > 150 => {
            *direction = Dir::from_value(direction.value() + 1);
            (x + 100, 49)
        }
        100..=149 if y < 0 => (199, x - 100),
        100..=149 if y >= 50 => {
            *direction = Dir::from_value(direction.value() + 1);
            (x - 50, 99)
        }
        _ => (y, x),
    }
}

fn wrap_x(direction: &mut Dir, (y, x): (isize, isize)) -> (isize, isize) {
    match y {
        0..=49 if x >= 150 => {
            *direction = Dir::from_value(direction.value() + 2);
            (149 - y, 99)
        }
        0..=49 if x < 50 => {
            *direction = Dir::from_value(direction.value() + 2);
            (149 - y, 0)
        }
        50..=99 if x >= 100 => {
            *direction = Dir::from_value(direction.value() + 3);
            (49, y + 50)
        }
        50..=99 if x < 50 => {
            *direction = Dir::from_value(direction.value() + 3);
            (100, y - 50)
        }
        100..=149 if x >= 100 => {
            *direction = Dir::from_value(direction.value() + 2);
            (149 - y, 149)
        }
        100..=149 if x < 0 => {
            *direction = Dir::from_value(direction.value() + 2);
            (149 - y, 50)
        }
        150..=199 if x < 0 => {
            *direction = Dir::from_value(direction.value() + 3);
            (0, y - 100)
        }
        150..=199 if x >= 50 => {
            *direction = Dir::from_value(direction.value() + 3);
            (149, y - 100)
        }
        _ => (y, x),
    }
}

fn to_usize((x, y): (isize, isize)) -> (usize, usize) {
    (x as usize, y as usize)
}

fn to_isize((x, y): (usize, usize)) -> (isize, isize) {
    (x as isize, y as isize)
}

fn parse_moves(line: &str) -> IResult<&str, Vec<Moves>> {
    complete(many0(alt((
        map(tag("L"), |_| Moves::Left),
        map(tag("R"), |_| Moves::Right),
        map(nom::character::complete::u32, Moves::Forward),
    ))))(line)
}

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Input {
    let mut iter = input.lines().rev();
    let mv = parse_moves(iter.next().unwrap()).unwrap().1;
    iter.next().unwrap();
    let board = iter.rev().map(|l| l.to_string().into_bytes()).collect();

    Input { mv, board }
}

#[aoc(day22, part1)]
pub fn part1(inputs: &Input) -> usize {
    let mut pos = inputs.find_start();
    let mut direction = Dir::Right;
    // println!("start: {pos:?}");

    for m in &inputs.mv {
        match m {
            Moves::Forward(n) => {
                pos = inputs.move_forward1(direction, *n, pos);
                // println!("move foward {n}, new: {pos:?}");
            }
            Moves::Right => {
                direction = direction.turn(&Moves::Right);
                // println!("turn right: Now {direction:?}")
            }
            Moves::Left => {
                direction = direction.turn(&Moves::Left);
                // println!("turn left: Now {direction:?}")
            }
        }
    }

    (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + direction.value()
}

#[aoc(day22, part2)]
pub fn part2(inputs: &Input) -> usize {
    let mut pos = to_isize(inputs.find_start());
    let mut direction = Dir::Right;
    // println!("start: {pos:?}");

    for m in &inputs.mv {
        match m {
            Moves::Forward(n) => {
                pos = inputs.move_forward2(&mut direction, *n, pos);
                // println!("move foward {n}, new: {pos:?}");
            }
            Moves::Right => {
                direction = direction.turn(&Moves::Right);
                // println!("turn right: Now {direction:?}")
            }
            Moves::Left => {
                direction = direction.turn(&Moves::Left);
                // println!("turn left: Now {direction:?}")
            }
        }
    }

    let pos = to_usize(pos);
    (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + direction.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("day22/SAMPLE.txt");

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 6032);
    }

    #[test]
    pub fn test2() {
        // assert_eq!(part2(&generator(SAMPLE)), 5031);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day22.txt");
        const ANSWERS: (usize, usize) = (26558, 110400);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            // assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
