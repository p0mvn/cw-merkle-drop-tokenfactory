use crate::hash;

/// TODO: spec and tests
pub fn build_leaf_level<T: AsRef<[u8]> + Ord>(items: &[T]) -> Vec<hash::Hash> {
    let mut nodes: Vec<hash::Hash> = Vec::with_capacity(calculate_tree_capacity(items));
    for item in items.iter() {
        let item = item.as_ref();
        let hash = hash::leaf(item);
        nodes.push(hash);
    }

    // sort items so that we can binary search them
    // when finding proofs.
    pdqsort::sort_by(&mut nodes, |a, b| a.cmp(b));

    // Duplicate the last entry if the number of items is odd
    // so that each parent has 2 children, and the tree is complete.
    if items.len() % 2 == 1 {
        let last_hash = nodes[nodes.len() - 1];
        nodes.push(last_hash);
    }

    return nodes;
}

pub fn build_branch_levels(nodes: &mut Vec<hash::Hash>) {
    let mut level_length = get_next_level_length(nodes.len());
    let mut level_start = 0;
    while level_length > 0 {
        for i in 0..level_length {
            let level_index = 2 * i;
            let left_sibling = &nodes[level_start + level_index];
            let right_sibling = &nodes[level_start + level_index + 1];

            let hash = hash::branch(left_sibling, right_sibling);
            nodes.push(hash);
        }
        level_start = level_length * 2;
        level_length = get_next_level_length(level_length);
    }
}

/// TODO: spec and tests
#[inline]
fn get_next_level_length(level_len: usize) -> usize {
    if level_len == 1 {
        0
    } else {
        (level_len + 1) / 2
    }
}

/// TODO: spec and tests
fn calculate_tree_capacity<T>(items: &[T]) -> usize {
    let leaves_count = items.len();
    let branch_node_count = round_up_power_of_two(items.len());
    return leaves_count + branch_node_count;
}

/// round_up_power_of_two returns the next power of two
/// https://graphics.stanford.edu/~seander/bithacks.html#RoundUpPowerOf2
/// TODO: test
fn round_up_power_of_two(n: usize) -> usize {
    let mut v = n;
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v += 1;
    v
}
