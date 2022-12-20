use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Node {
    val: isize,
    pos: usize,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Vec<Node> {
    input
        .lines()
        .enumerate()
        .map(|(pos, val)| Node {
            pos,
            val: val.parse().unwrap(),
        })
        .collect()
}

#[aoc(day20, part1)]
pub fn part1(inputs: &[Node]) -> isize {
    let mut data = inputs.to_vec();

    for &node in inputs.iter() {
        let idx = data.iter().position(|&x| x == node).unwrap();
        data.remove(idx);

        let idx = ((idx as isize) + node.val).rem_euclid(data.len() as isize) as usize;
        if idx == 0 {
            data.push(node);
        } else {
            data.insert(idx, node);
        };

        // println!("moving {} to {idx}:{data:?}", node.val);
    }

    let zero_idx = data.iter().position(|x| x.val == 0).unwrap();

    let mut total = 0;
    for i in [1000, 2000, 3000] {
        let idx = (zero_idx + i) % data.len();
        total += data[idx].val;
    }

    total
}

#[aoc(day20, part2)]
pub fn part2(inputs: &[Node]) -> isize {
    let mut data = inputs
        .iter()
        .map(|x| Node {
            val: x.val * 811589153,
            pos: x.pos,
        })
        .collect::<Vec<_>>();
    let inputs = data.clone();

    for _ in 0..10 {
        for &node in inputs.iter() {
            let idx = data.iter().position(|&x| x == node).unwrap();
            data.remove(idx);

            let idx = ((idx as isize) + node.val).rem_euclid(data.len() as isize) as usize;

            if idx == 0 {
                data.push(node);
            } else {
                data.insert(idx, node);
            };
            // println!("moving {} to {idx}:{data:?}", node.val);
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
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 3);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 1623178306);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day20.txt");
        const ANSWERS: (isize, isize) = (7153, 6146976244822);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
