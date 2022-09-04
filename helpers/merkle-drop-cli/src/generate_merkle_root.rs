use hex;

// use merkle_light::merkle::{MerkleTree};
use solana_merkle_tree::{MerkleTree};

pub fn run(data: Vec<Vec<u8>>) -> String {
    // let t: MerkleTree<Leaf, Sha256> = MerkleTree::from_data(data);
    let merkle_tree = MerkleTree::new(&data);
    let hash = merkle_tree.get_root().unwrap();

    hex::encode(hash.to_bytes())
}
