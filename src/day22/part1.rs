use super::{Dir, Input};

impl Dir {
    fn forward_wrap(&self, (y, x): (usize, usize), y_max: usize, x_max: usize) -> (usize, usize) {
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
}

pub(super) fn move_forward(
    input: &Input,
    direction: &mut Dir,
    n: u32,
    mut pos: (usize, usize),
) -> (usize, usize) {
    let h = input.board.len();
    let w = input.board[0].len();

    for _ in 0..n {
        let new_pos = direction.forward_wrap(pos, h, w);
        match input.board_get(new_pos) {
            Some(b'#') => {
                break;
            }
            Some(_) => {
                pos = new_pos;
            }
            None => {
                // Move forward 1 step until we wrap around and board_get succeeds
                pos = {
                    let mut temp = pos;
                    loop {
                        temp = direction.forward_wrap(temp, h, w);
                        if input.board_get(temp).is_some() {
                            break temp;
                        }
                    }
                };

                continue;
            }
        }
    }

    pos
}
