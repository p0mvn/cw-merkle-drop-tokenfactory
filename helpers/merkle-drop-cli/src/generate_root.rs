use hex;

use merkle::Tree;

pub fn run(data: Vec<Vec<u8>>) -> String {
    let merkle_tree = Tree::new(&data);
    let hash = merkle_tree.get_root().unwrap();

    return hex::encode(hash);
}
