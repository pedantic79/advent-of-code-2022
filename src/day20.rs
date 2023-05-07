use aoc_runner_derive::{aoc, aoc_generator};
use nom::combinator::map;

use crate::common::nom::{nom_i64, nom_lines, process_input};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Node {
    val: i64,
    pos: usize,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Vec<Node> {
    let mut pos = 0;

    let x = process_input(nom_lines(map(nom_i64::<_, ()>, |val| {
        let r = Node { pos, val };
        pos += 1;
        r
    })))(input);

    // We need to bind and return to avoid `pos` incorrectly "borrowed value does not live long enough"
    #[allow(clippy::let_and_return)]
    x
}

fn solve<const ITERATIONS: usize>(inputs: &[Node]) -> i64 {
    let mut data = inputs.to_vec();

    for _ in 0..ITERATIONS {
        for &node in inputs.iter() {
            let pos = data.iter().position(|&x| x == node).unwrap();
            let idx = ((pos as i64) + node.val).rem_euclid((data.len() - 1) as i64) as usize;

            if idx != pos {
                data.remove(pos);

                if idx == 0 {
                    data.push(node);
                } else {
                    data.insert(idx, node);
                };

                // println!("moving {} to {idx}:{data:?}", node.val);
            }
        }
    }

    let zero_idx = data.iter().position(|x| x.val == 0).unwrap();

    let mut total = 0;
    for i in [1000, 2000, 3000] {
        let idx = (zero_idx + i) % data.len();
        total += data[idx].val;
    }

    total
}

#[aoc(day20, part1)]
pub fn part1(inputs: &[Node]) -> i64 {
    solve::<1>(inputs)
}

#[aoc(day20, part2)]
pub fn part2(inputs: &[Node]) -> i64 {
    let data = inputs
        .iter()
        .map(|x| Node {
            val: x.val * 811589153,
            pos: x.pos,
        })
        .collect::<Vec<_>>();

    solve::<10>(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"1
2
-3
3
-2
0
4";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 3);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 1623178306);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day20.txt");
        const ANSWERS: (i64, i64) = (7153, 6146976244822);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
