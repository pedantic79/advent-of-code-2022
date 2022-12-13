use std::{collections::VecDeque, mem::swap};

use bit_set::BitSet;

/// `bfs_count_bitset` returns the number of elements and using a bitset to keep track of seen nodes.
/// Based on [pathfinding](https://github.com/samueltardieu/pathfinding/blob/v4.0.0/src/directed/bfs.rs#L78)
/// `bfs` algorithm.
///
/// - `mapper`, must map to a usize. This is useful to map coordinates, but not hashing.
pub fn bfs_count_bitset<N, FN, IN, FS, FM>(
    start: &N,
    mut successors: FN,
    mut success: FS,
    mut mapper: FM,
) -> Option<usize>
where
    N: Eq + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
    FM: FnMut(&N) -> usize,
{
    if success(start) {
        return Some(1);
    }
    let mut depth = 0;
    let mut queue_now = VecDeque::new();
    let mut queue_new = VecDeque::new();
    let mut seen = BitSet::new();

    queue_now.push_back(start.clone());
    seen.insert(mapper(start));
    loop {
        depth += 1;
        while let Some(node) = queue_now.pop_front() {
            for successor in successors(&node) {
                if success(&successor) {
                    return Some(depth);
                }
                let v = mapper(&successor);
                if !seen.contains(v) {
                    queue_new.push_back(successor);
                    seen.insert(v);
                }
            }
        }

        if queue_new.is_empty() {
            break None;
        } else {
            swap(&mut queue_now, &mut queue_new);
        }
    }
}
