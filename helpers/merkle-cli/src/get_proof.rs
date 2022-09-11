use hex;

use solana_merkle_tree::MerkleTree;
use solana_program::hash::{hashv, Hash};

const LEAF_PREFIX: &[u8] = &[0];

macro_rules! hash_leaf {
    {$d:ident} => {
        hashv(&[LEAF_PREFIX, $d])
    }
}

pub fn run(data: Vec<Vec<u8>>, address: String, amount: String) {
    let merkle_tree = MerkleTree::new(&data);

    // let combined: &[u8] = format!("{}{}", address, amount).as_bytes();

    // let hashed = hash_leaf!(combined);

    // merkle_tree.find_path(hashed.to_bytes());
    return;
}
