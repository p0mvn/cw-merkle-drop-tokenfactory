use crate::hash;
use crate::binary_search;
use crate::builder;
use crate::proof;

#[derive(Debug)]
pub struct MerkleTree {
    leaf_count: usize,
    nodes: Vec<hash::Hash>,
}

impl MerkleTree {
    pub fn new<T: AsRef<[u8]>>(items: &[T]) -> Self {
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

    pub fn find_proof<T: AsRef<[u8]>>(&self, item: &T) -> Option<proof::Proof> {
        if self.leaf_count <= 1 {
            return None;
        }

        let item_ref = item.as_ref();
        let hash_to_search_for = hash::leaf(item_ref);

        // binary search leaves
        let search_result = binary_search::search(&self.nodes, self.leaf_count, &hash_to_search_for);
        if search_result.is_none() {
            return None;
        }

        let proof = proof::Proof::default();

        let proof_index = search_result.unwrap();

        let mut level_length = self.leaf_count;
        let mut level_start = 0; 
        let mut current_index = proof_index;
        let mut current_node: hash::Hash;

        while level_length != 1 {
            current_node = self.nodes[level_start + current_index];

            // if index is odd, grab index - 1 for sibling
            // if index is even, graab index + 1 for singling
               // if level_length is odd, grab itself for sibgling

            level_start += level_length;
            level_length = builder::get_next_level_length(level_length);
            current_index = current_index / 2;
        }

        Some(proof)

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
        let items: Vec<&[u8]> = vec![test_util::OSMO];

        let mt = MerkleTree::new(&items);

        let root = mt.get_root();

        assert_eq!(true, root.is_some());
        assert_eq!(1, mt.get_leaf_count().unwrap());
        assert_eq!(1, mt.get_node_count());

        // TODO: extra this into helper and clean up tests
        match mt.get_node_at(0) {
            Ok(result) => {
                assert_eq!(hash::leaf(test_util::OSMO), result);
                assert_eq!(root.unwrap(), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }
    }

    #[test]
    fn new_merkle_tree_two_elements() {
        let mut items:Vec<&[u8]> = vec![test_util::OSMO, test_util::ION];

        let mt = MerkleTree::new(&items);

        test_util::hash_and_sort(&mut items);

        let root = mt.get_root();

        assert_eq!(true, root.is_some());
        assert_eq!(2, mt.get_leaf_count().unwrap());
        assert_eq!(3, mt.get_node_count());

        match mt.get_node_at(0) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[0]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(1) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[1]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(2) {
            Ok(result) => {
                let left_hash: hash::Hash = hash::leaf(items[0]);
                let right_hash: hash::Hash = hash::leaf(items[1]);
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
        let mut items: Vec<&[u8]> = vec![test_util::OSMO, test_util::WETH, test_util::ION];

        let mt = MerkleTree::new(&items);

        test_util::hash_and_sort(&mut items);

        let root = mt.get_root();

        assert_eq!(true, root.is_some());
        assert_eq!(3, mt.get_leaf_count().unwrap());
        assert_eq!(6, mt.get_node_count());

        match mt.get_node_at(0) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[0]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(1) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[1]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(2) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[2]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(3) {
            Ok(result) => {
                let left_hash: hash::Hash = hash::leaf(items[0]);
                let right_hash: hash::Hash = hash::leaf(items[1]);
                assert_eq!(hash::branch(&left_hash, &right_hash), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(4) {
            Ok(result) => {
                let left_hash: hash::Hash = hash::leaf(items[2]);
                let right_hash: hash::Hash = hash::leaf(items[2]);
                assert_eq!(hash::branch(&left_hash, &right_hash), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(5) {
            Ok(result) => {
                assert_eq!(mt.get_root().unwrap(), result);

                let left_left_hash: hash::Hash = hash::leaf(items[0]);
                let left_right_hash: hash::Hash = hash::leaf(items[1]);

                let left_hash: hash::Hash = hash::branch(&left_left_hash, &left_right_hash);
                assert_eq!(hash::branch(&left_left_hash, &left_right_hash), left_hash);

                let right_left_hash: hash::Hash = hash::leaf(items[2]);
                let right_right_hash: hash::Hash = hash::leaf(items[2]);

                let right_hash: hash::Hash = hash::branch(&right_left_hash, &right_right_hash);
                assert_eq!(hash::branch(&right_left_hash, &right_right_hash), right_hash);

                assert_eq!(hash::branch(&left_hash, &right_hash), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }
    }

    #[test]
    fn find_proof_one() {
        let items: Vec<&[u8]> = vec![test_util::OSMO];

        let mt = MerkleTree::new(&items);

        let result = mt.find_proof(&test_util::OSMO);

        assert_eq!(true, result.is_none());
    }

    // #[test]
    // fn find_proof_two() {

    // }


}

#[cfg(test)]
pub mod test_util {
    use super::*;

    pub const OSMO: &[u8] = b"osmo";
    pub const ION: &[u8] = b"ion";
    pub const WETH: &[u8] = b"weth";
    pub const USDC: &[u8] = b"usdc";
    pub const AKT: &[u8] = b"akt";

    pub fn hash_and_sort(items: &mut Vec<&[u8]>) {
        // We expect the constructor to sort the nodes by hash.
        pdqsort::sort_by(items, |a, b| {
            hash::leaf(a).cmp(&hash::leaf(b))
        });
    }

    pub fn sort(items: &mut Vec<hash::Hash>) {
        // We expect the constructor to sort the nodes by hash.
        pdqsort::sort_by(items, |a, b| {
            a.cmp(b)
        });
    }
}
