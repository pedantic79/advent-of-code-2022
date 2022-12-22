use super::{Dir, Input};

impl Dir {
    fn forward_checked(&self, (y, x): (usize, usize)) -> (Option<usize>, Option<usize>) {
        match self {
            Dir::Left => (Some(y), x.checked_sub(1)),
            Dir::Right => (Some(y), Some(x + 1)),
            Dir::Up => (y.checked_sub(1), Some(x)),
            Dir::Down => (Some(y + 1), Some(x)),
        }
    }
}

fn wrap_y(direction: &mut Dir, pos: (Option<usize>, Option<usize>)) -> (usize, usize) {
    let (y, x) = (pos.0.unwrap_or_default(), pos.1.unwrap_or_default());

    match x {
        0..=49 if y < 100 => {
            *direction = direction.increment(1);
            (x + 50, 50)
        }
        0..=49 if y >= 200 => (0, x + 100),
        50..=99 if pos.0.is_none() => {
            *direction = direction.increment(1);
            (x + 100, 0)
        }
        50..=99 if y > 150 => {
            *direction = direction.increment(1);
            (x + 100, 49)
        }
        100..=149 if pos.0.is_none() => (199, x - 100),
        100..=149 if y >= 50 => {
            *direction = direction.increment(1);
            (x - 50, 99)
        }
        _ => (y, x),
    }
}

fn wrap_x(direction: &mut Dir, pos: (Option<usize>, Option<usize>)) -> (usize, usize) {
    let (y, x) = (pos.0.unwrap_or_default(), pos.1.unwrap_or_default());

    match y {
        0..=49 if x >= 150 => {
            *direction = direction.increment(2);
            (149 - y, 99)
        }
        0..=49 if x < 50 => {
            *direction = direction.increment(2);
            (149 - y, 0)
        }
        50..=99 if x >= 100 => {
            *direction = direction.increment(3);
            (49, y + 50)
        }
        50..=99 if x < 50 => {
            *direction = direction.increment(3);
            (100, y - 50)
        }
        100..=149 if x >= 100 => {
            *direction = direction.increment(2);
            (149 - y, 149)
        }
        100..=149 if pos.1.is_none() => {
            *direction = direction.increment(2);
            (149 - y, 50)
        }
        150..=199 if pos.1.is_none() => {
            *direction = direction.increment(3);
            (0, y - 100)
        }
        150..=199 if x >= 50 => {
            *direction = direction.increment(3);
            (149, y - 100)
        }
        _ => (y, x),
    }
}

pub(super) fn move_forward(
    input: &Input,
    direction: &mut Dir,
    n: u32,
    mut pos: (usize, usize),
) -> (usize, usize) {
    for _ in 0..n {
        let new_pos = direction.forward_checked(pos);
        let mut new_direction = *direction;

        let new_pos = match direction {
            Dir::Up | Dir::Down => wrap_y(&mut new_direction, new_pos),
            Dir::Left | Dir::Right => wrap_x(&mut new_direction, new_pos),
        };

        match input.board_get(new_pos) {
            Some(b'#') | None => {}
            Some(_) => {
                pos = new_pos;
                *direction = new_direction;
            }
        }
    }

    pos
}
