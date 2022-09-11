use hex;

use serde_json;
use merkle::Tree;
use merkle::hash;
use std::error::Error;

pub fn run(data: &Vec<Vec<u8>>) -> String {
    let tree = Tree::new(data);
    let hash = tree.get_root().unwrap();

    return hex::encode(hash);
}

pub fn get_proof(data: &Vec<Vec<u8>>, proof_for: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    let tree = Tree::new(data);

    let proof_opt = tree.find_proof(proof_for);

    if proof_opt.is_none() {
       return Err(format!("failed to find proof for {:?}, the data hash is {:?}", proof_for, hash::leaf(proof_for)).into());
    }

    let proof = proof_opt.unwrap();

    let serialized = serde_json::to_string(&proof)?;

    Ok(serialized)
}

pub fn hash(data: &String) -> String {
    return merkle::hash::leaf(data.as_bytes()).to_string();
}
