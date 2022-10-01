use cosmwasm_std::{DepsMut};
use merkle::{proof::Proof, hash::Hash};

use crate::{state::CONFIG, ContractError};

pub fn verify_proof(deps: DepsMut, proof_str: &String, to_verify: &String) -> Result<(), ContractError> {
    let proof: Proof = serde_json::from_str(proof_str).unwrap();

    let root_encoded = CONFIG.load(deps.storage).unwrap().merkle_root;

    let root = match base64::decode(root_encoded) {
        Ok(f)=> {
            f
         },
         Err(e)=> {
            return Err(ContractError::FailedToDecodeRoot { root: e.to_string() })
         }
    };

    let root_hash = Hash::from(root);

    if !proof.verify(to_verify, &root_hash) {
        return Err(ContractError::FailedVerifyProof {  })
    }

    Ok(())
}
