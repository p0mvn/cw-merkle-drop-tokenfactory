use crate::hash;

/// TODO: spec and tests
pub fn build_leaf_level<T: AsRef<[u8]>>(items: &[T]) -> Vec<hash::Hash> {
    let mut nodes: Vec<hash::Hash> = Vec::with_capacity(calculate_tree_capacity(items));
    for item in items.iter() {
        let item = item.as_ref();
        let hash = hash::leaf(item);
        nodes.push(hash)
    }

    // sort items so that we can binary search them
    // when finding proofs.
    pdqsort::sort_by(&mut nodes, |a, b| a.cmp(b));

    return nodes;
}

// build_branch_levels from nodes.
// CONTRACT: nodes are sorted in incrasing order.
pub fn build_branch_levels(nodes: &mut Vec<hash::Hash>) {
    let mut previous_level_length = nodes.len();
    let mut current_level_length = get_next_level_length(previous_level_length);
    let mut previous_level_start = 0;
    while current_level_length > 0 {
        for i in 0..current_level_length {
            let previous_level_index = 2 * i;
            let nodes_index: usize = previous_level_start +previous_level_index;
            let left_sibling = &nodes[nodes_index];

            let right_sibling = if previous_level_index + 1 >= previous_level_length {
                &nodes[nodes_index] // For the case where the number of nodes at a level is odd.
            } else {
                &nodes[nodes_index + 1]
            };

            let hash = hash::branch(left_sibling, right_sibling);
            nodes.push(hash);
        }
        previous_level_start += previous_level_length;
        previous_level_length = current_level_length;
        current_level_length = get_next_level_length(current_level_length);
    }
}

/// TODO: spec and tests
#[inline]
pub fn get_next_level_length(level_len: usize) -> usize {
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const OSMO: &[u8] = b"osmo";
    const ION: &[u8] = b"ion";
    const WETH: &[u8] = b"weth";
    const USDC: &[u8] = b"usdc";
    const AKT: &[u8] = b"akt";

    #[test]
    fn build_branch_level_one_node() {
        let items: Vec<&[u8]> = vec![OSMO];

        let mut actual_nodes: Vec<hash::Hash> = prepare_leaf_nodes(&items);
        let expected_nodes: Vec<hash::Hash> = actual_nodes.clone();

        build_branch_levels(&mut actual_nodes);

        validate_nodes(&expected_nodes, &actual_nodes);
    }

    #[test]
    fn build_branch_level_two_nodes() {
        let items: Vec<&[u8]> = vec![OSMO, ION];

        let mut actual_nodes: Vec<hash::Hash> = prepare_leaf_nodes(&items);

        let mut expected_nodes: Vec<hash::Hash> = actual_nodes.clone();
        expected_nodes.push(hash::branch(&expected_nodes[0], &expected_nodes[1]));

        build_branch_levels(&mut actual_nodes);

        validate_nodes(&expected_nodes, &actual_nodes);
    }

    #[test]
    fn build_branch_level_three_nodes() {
        let items: Vec<&[u8]> = vec![OSMO, ION, WETH];

        let mut actual_nodes: Vec<hash::Hash> = prepare_leaf_nodes(&items);

        let mut expected_nodes: Vec<hash::Hash> = actual_nodes.clone();
        expected_nodes.push(hash::branch(&expected_nodes[0], &expected_nodes[1]));
        expected_nodes.push(hash::branch(&expected_nodes[2], &expected_nodes[2]));
        expected_nodes.push(hash::branch(&expected_nodes[3], &expected_nodes[4]));

        build_branch_levels(&mut actual_nodes);

        validate_nodes(&expected_nodes, &actual_nodes);
    }

    #[test]
    fn build_branch_level_five_nodes() {
        let items: Vec<&[u8]> = vec![OSMO, ION, WETH, USDC, AKT];

        let mut actual_nodes: Vec<hash::Hash> = prepare_leaf_nodes(&items);

        let mut expected_nodes: Vec<hash::Hash> = actual_nodes.clone();
        // level 3
        expected_nodes.push(hash::branch(&expected_nodes[0], &expected_nodes[1]));
        expected_nodes.push(hash::branch(&expected_nodes[2], &expected_nodes[3]));
        expected_nodes.push(hash::branch(&expected_nodes[4], &expected_nodes[4]));

        // level 2
        expected_nodes.push(hash::branch(&expected_nodes[5], &expected_nodes[6]));
        expected_nodes.push(hash::branch(&expected_nodes[7], &expected_nodes[7]));

        // level 1
        expected_nodes.push(hash::branch(&expected_nodes[8], &expected_nodes[9]));

        build_branch_levels(&mut actual_nodes);

        validate_nodes(&expected_nodes, &actual_nodes);
    }

    fn sort(items: &mut Vec<hash::Hash>) {
        // We expect the constructor to sort the nodes by hash.
        pdqsort::sort_by(items, |a, b| {
            a.cmp(b)
        });
    }

    fn prepare_leaf_nodes(items: &Vec<&[u8]>) -> Vec<hash::Hash> {
        let mut actual_nodes: Vec<hash::Hash> = items.into_iter().map(|i| hash::leaf(i)).rev().collect();

        sort(&mut actual_nodes);
        return  actual_nodes;
    }
    
    fn validate_nodes(expected_nodes: &Vec<hash::Hash>, actual_nodes: &Vec<hash::Hash>) {
        assert_eq!(expected_nodes.len(), actual_nodes.len());
        for i in 0..actual_nodes.len() {
            assert_eq!(expected_nodes[i], actual_nodes[i], "index {}", i);
        }
    }
}
