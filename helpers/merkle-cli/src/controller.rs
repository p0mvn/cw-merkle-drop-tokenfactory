use base64;

use merkle::hash;
use merkle::Tree;
use serde_json_wasm;
use std::error::Error;

pub fn generate_root(data: &Vec<Vec<u8>>) -> String {
    let tree = Tree::new(data);
    let hash = tree.get_root().unwrap();

    return base64::encode(hash);
}

pub fn get_proof(data: &Vec<Vec<u8>>, proof_for: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    let tree = Tree::new(data);

    let proof_opt = tree.find_proof(proof_for);

    if proof_opt.is_none() {
        return Err(format!(
            "failed to find proof for {:?}, the data hash is {:?}",
            proof_for,
            hash::leaf(proof_for)
        )
        .into());
    }

    let proof = proof_opt.unwrap();

    let serialized = serde_json_wasm::to_string(&proof)?;

    Ok(serialized)
}

pub fn verify_proof(
    root: &String,
    proof_bytes: &String,
    to_verify: &String,
) -> Result<bool, Box<dyn Error>> {
    let proof: merkle::proof::Proof = serde_json_wasm::from_str(&proof_bytes)?;
    let root_decoded = base64::decode(root)?;

    proof.verify(to_verify, &merkle::hash::Hash::from(root_decoded));

    Ok(true)
}

pub fn hash(data: &String) -> String {
    return merkle::hash::leaf(data.as_bytes()).to_string();
}
