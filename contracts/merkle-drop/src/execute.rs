use merkle::{hash::Hash, proof::Proof};

use crate::ContractError;

pub fn verify_proof(
    merkle_root: &String,
    proof_str: &String,
    to_verify: &String,
) -> Result<(), ContractError> {
    let proof: Proof = serde_json_wasm::from_str(proof_str).unwrap();
    let root = match base64::decode(merkle_root) {
        Ok(f) => f,
        Err(e) => {
            return Err(ContractError::FailedToDecodeRoot {
                root: e.to_string(),
            })
        }
    };

    let root_hash = Hash::from(root);

    if !proof.verify(to_verify, &root_hash) {
        return Err(ContractError::FailedVerifyProof {});
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_ROOT test merkel root that was generated from "testdata/uosmo_only.csv" using merkle-drop-cli
    const TEST_ROOT: &str = "Nz54SQtyBVHwsmEqNI//mxFgiq8MRD7sS92IGkhgMvo=";

    const VALID_PROOF_STR: &str = "[{\"is_left_sibling\":true,\"hash\":[77,122,52,81,83,57,77,56,52,118,78,84,89,115,86,76,120,80,121,50,109,89,49,111,107,85,111,68,111,108,97,118,105,81,119,71,114,54,111,49,51,107,99,61]},{\"is_left_sibling\":true,\"hash\":[113,53,102,108,80,98,70,114,73,121,100,108,105,97,121,52,85,73,57,100,107,111,106,71,82,116,49,57,90,71,121,105,51,111,55,104,103,112,87,103,73,117,48,61]},{\"is_left_sibling\":true,\"hash\":[114,87,52,69,82,120,75,113,110,53,102,77,69,65,114,57,56,118,90,80,116,54,119,108,67,71,49,69,80,121,99,57,51,54,108,48,112,100,86,97,100,120,48,61]},{\"is_left_sibling\":false,\"hash\":[66,86,43,54,113,43,104,43,100,99,115,116,50,83,66,52,122,68,111,100,118,50,98,90,84,112,108,105,105,65,104,87,50,82,75,121,112,67,72,51,81,66,85,61]},{\"is_left_sibling\":false,\"hash\":[86,100,75,79,107,112,115,120,78,73,103,43,50,111,102,47,57,76,84,111,122,51,107,84,102,53,112,90,113,81,108,108,112,86,84,86,82,55,108,77,103,83,119,61]}]";

    const INVALID_PROOF_STR: &str = "[{\"is_left_sibling\":true,\"hash\":[78,122,52,81,83,57,77,56,52,118,78,84,89,115,86,76,120,80,121,50,109,89,49,111,107,85,111,68,111,108,97,118,105,81,119,71,114,54,111,49,51,107,99,61]},{\"is_left_sibling\":true,\"hash\":[113,53,102,108,80,98,70,114,73,121,100,108,105,97,121,52,85,73,57,100,107,111,106,71,82,116,49,57,90,71,121,105,51,111,55,104,103,112,87,103,73,117,48,61]},{\"is_left_sibling\":true,\"hash\":[114,87,52,69,82,120,75,113,110,53,102,77,69,65,114,57,56,118,90,80,116,54,119,108,67,71,49,69,80,121,99,57,51,54,108,48,112,100,86,97,100,120,48,61]},{\"is_left_sibling\":false,\"hash\":[66,86,43,54,113,43,104,43,100,99,115,116,50,83,66,52,122,68,111,100,118,50,98,90,84,112,108,105,105,65,104,87,50,82,75,121,112,67,72,51,81,66,85,61]},{\"is_left_sibling\":false,\"hash\":[86,100,75,79,107,112,115,120,78,73,103,43,50,111,102,47,57,76,84,111,122,51,107,84,102,53,112,90,113,81,108,108,112,86,84,86,82,55,108,77,103,83,119,61]}]";

    const TO_VERIFY_VALID: &str = "osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo";

    #[test]
    fn verify_proof_success() {
        verify_proof(
            &String::from(TEST_ROOT),
            &String::from(VALID_PROOF_STR),
            &String::from(TO_VERIFY_VALID),
        )
        .unwrap();
    }

    #[test]
    fn verify_proof_invalid_root_error() {
        verify_proof(
            &String::from("this is garbage"),
            &String::from(VALID_PROOF_STR),
            &String::from(TO_VERIFY_VALID),
        )
        .unwrap_err();
    }

    #[test]
    fn verify_proof_invalid_proof_error() {
        verify_proof(
            &String::from(TEST_ROOT),
            &String::from(INVALID_PROOF_STR),
            &String::from(TO_VERIFY_VALID),
        )
        .unwrap_err();
    }
}
