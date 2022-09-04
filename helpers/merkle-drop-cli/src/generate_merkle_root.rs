use hex;

use solana_merkle_tree::{MerkleTree};

pub fn run(data: Vec<Vec<u8>>) -> String {
    let merkle_tree = MerkleTree::new(&data);
    let hash = merkle_tree.get_root().unwrap();

    hex::encode(hash.to_bytes())
}
