use crate::hash;
use crate::binary_search;
use crate::builder;

#[derive(Debug)]
pub struct MerkleTree {
    leaf_count: usize,
    nodes: Vec<hash::Hash>,
}

impl MerkleTree {
    pub fn new<T: AsRef<[u8]> + Ord>(items: &[T]) -> Self {
        if items.len() == 0 {
            return MerkleTree {
                leaf_count: 0,
                nodes: Vec::<hash::Hash>::new(),
            };
        }

        let mut nodes: Vec<hash::Hash> = builder::build_leaf_level(items);
        let leaf_count = nodes.len();

        builder::build_branch_levels(&mut nodes);

        let mt = MerkleTree {
            leaf_count: leaf_count,
            nodes: nodes,
        };
        mt
    }

    pub fn get_root(&self) -> Option<hash::Hash> {
        let node_count = self.nodes.len();
        if self.leaf_count == 0 {
            return None;
        }
        let root_copy = self.nodes[node_count - 1].clone();
        Some(root_copy)
    }

    pub fn find_proof<T: AsRef<[u8]>>(&self, item: &T) {
        let item_ref = item.as_ref();
        let hash_to_search_for = hash::leaf(item_ref);

        // binary search leaves
        let search_result = binary_search::search(&self.nodes, self.leaf_count, &hash_to_search_for);
        if search_result.is_none() {}
    }

    fn get_node_count(&self) -> usize {
        return self.nodes.len();
    }

    fn get_leaf_count(&self) -> Result<usize, String> {
        if self.leaf_count > self.get_node_count() {
            return Err(format!(
                "leaf count ({}) is greater than node count ({})",
                self.leaf_count,
                self.get_node_count()
            ));
        }
        return Ok(self.leaf_count);
    }

    fn get_node_at(&self, index: usize) -> Result<hash::Hash, String> {
        if index >= self.get_node_count() {
            return Err(format!(
                "requested index ({}) is greater than node count ({})",
                index,
                self.get_node_count()
            ));
        }
        return Ok(self.nodes[index]);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    /// Merkle Tree Tests
    ////////////////////////////////////////////////
    const OSMO: &[u8] = b"osmo";
    const ION: &[u8] = b"ion";
    const WETH: &[u8] = b"weth";

    #[test]
    fn new_merkle_tree_empty() {
        let items: Vec<String> = vec![];

        let mt = MerkleTree::new(&items);

        let root = mt.get_root();

        assert_eq!(false, root.is_some());
        assert_eq!(0, mt.get_leaf_count().unwrap());
        assert_eq!(0, mt.get_node_count());

        match mt.get_node_at(0) {
            Ok(result) => {
                panic!("must have returned error but received {:?}", result)
            }
            Err(_error) => {
                // expected
            }
        }
    }

    #[test]
    fn new_merkle_tree_one_element() {
        let items: Vec<&[u8]> = vec![OSMO];

        let mt = MerkleTree::new(&items);

        let root = mt.get_root();

        assert_eq!(true, root.is_some());
        assert_eq!(2, mt.get_leaf_count().unwrap());
        assert_eq!(3, mt.get_node_count());

        // TODO: extra this into helper and clean up tests
        match mt.get_node_at(0) {
            Ok(result) => {
                assert_eq!(hash::leaf(OSMO), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(1) {
            Ok(result) => {
                assert_eq!(hash::leaf(OSMO), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(2) {
            Ok(result) => {
                assert_eq!(hash::branch(&hash::leaf(OSMO), &hash::leaf(OSMO)), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }
    }

    #[test]
    fn new_merkle_tree_two_elements() {
        let mut items: Vec<&str> = vec!["osmo", "ion"];

        let mt = MerkleTree::new(&items);

        sort(&mut items);

        let root = mt.get_root();

        assert_eq!(true, root.is_some());
        assert_eq!(2, mt.get_leaf_count().unwrap());
        assert_eq!(3, mt.get_node_count());

        match mt.get_node_at(0) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[0].as_bytes()), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(1) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[1].as_bytes()), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(2) {
            Ok(result) => {
                let left_hash: hash::Hash = hash::leaf(items[0].as_bytes());
                let right_hash: hash::Hash = hash::leaf(items[1].as_bytes());
                assert_eq!(hash::branch(&left_hash, &right_hash), result);
                assert_eq!(mt.get_root().unwrap(), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }
    }

    #[test]
    fn new_merkle_tree_three_elements() {
        let mut items: Vec<&str> = vec!["osmo", "weth", "ion"];

        let mt = MerkleTree::new(&items);

        sort(&mut items);

        let root = mt.get_root();

        assert_eq!(true, root.is_some());
        assert_eq!(4, mt.get_leaf_count().unwrap());
        assert_eq!(7, mt.get_node_count());

        match mt.get_node_at(0) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[0].as_bytes()), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(1) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[1].as_bytes()), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(2) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[2].as_bytes()), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(3) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[2].as_bytes()), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(4) {
            Ok(result) => {
                let left_hash: hash::Hash = hash::leaf(items[0].as_bytes());
                let right_hash: hash::Hash = hash::leaf(items[1].as_bytes());
                assert_eq!(hash::branch(&left_hash, &right_hash), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(5) {
            Ok(result) => {
                let left_hash: hash::Hash = hash::leaf(items[2].as_bytes());
                let right_hash: hash::Hash = hash::leaf(items[2].as_bytes());
                assert_eq!(hash::branch(&left_hash, &right_hash), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(6) {
            Ok(result) => {
                let left_left_hash: hash::Hash = hash::leaf(items[0].as_bytes());
                let left_right_hash: hash::Hash = hash::leaf(items[1].as_bytes());

                let left_hash: hash::Hash = hash::branch(&left_left_hash, &left_right_hash);
                assert_eq!(hash::branch(&left_left_hash, &left_right_hash), left_hash);

                let right_left_hash: hash::Hash = hash::leaf(items[2].as_bytes());
                let right_right_hash: hash::Hash = hash::leaf(items[2].as_bytes());

                let right_hash: hash::Hash = hash::branch(&right_left_hash, &right_right_hash);
                assert_eq!(hash::branch(&right_left_hash, &right_right_hash), right_hash);

                assert_eq!(hash::branch(&left_hash, &right_hash), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }
    }

    fn sort(items: &mut Vec<&str>) {
        // We expect the constructor to sort the nodes by hash.
        pdqsort::sort_by(items, |a, b| {
            hash::leaf(a.as_bytes()).cmp(&hash::leaf(b.as_bytes()))
        });
    }
}
