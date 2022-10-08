use cosmwasm_std::{DepsMut, Env, Response, SubMsg, Uint128};
use merkle::{hash::Hash, proof::Proof};
use osmosis_std::shim::Any;
use osmosis_std::types::cosmos::authz::v1beta1::MsgExec;
use osmosis_std::types::cosmos::base::v1beta1;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgMint, TokenfactoryQuerier};

use crate::error::ContractError;
use crate::reply::AUTHZ_EXEC_MINT_MSG_ID;
use crate::state::{MintReplyState, CLAIMED_ADDRESSES, CONFIG, REPLY_STATE, SUBDENOM};

pub fn claim(
    deps: DepsMut,
    env: Env,
    proof_str: String,
    amount: Uint128,
    claimer_addr: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage).unwrap();

    // TODO: validate claimer_addr is an actual account

    let claim = format!("{}{}", claimer_addr, amount);

    let claim_check = CLAIMED_ADDRESSES.may_load(deps.storage, &claim)?;
    if claim_check.is_some() {
        return Err(ContractError::AlreadyClaimed { claim });
    }

    deps.api
        .debug(&format!("merkle_root {0}", &config.merkle_root));

    deps.api.debug(&format!("proof_str {0}", &proof_str));

    deps.api.debug(&format!("claim {0}", &claim));

    verify_proof(&config.merkle_root, &proof_str, claim)?;

    deps.api.debug("validation passed");

    let subdenom = SUBDENOM.load(deps.storage)?;

    let full_denom = format!("factory/{}/{}", config.owner, subdenom);
    deps.api
        .debug(&format!("claim full_denom: claim end: {}", full_denom));

    let tf_querier = TokenfactoryQuerier::new(&deps.querier);
    let admin = tf_querier
        .denom_authority_metadata(full_denom.clone())?
        .authority_metadata
        .unwrap()
        .admin;
    deps.api.debug(&format!("denom admin = {admin:?}"));

    let mint_msg_res = MsgMint {
        sender: config.owner.to_string(),
        amount: Some(v1beta1::Coin {
            denom: full_denom.clone(),
            amount: amount.to_string(),
        }),
    };

    let mint_msg_binary: cosmwasm_std::Binary = mint_msg_res.into();

    let mint_msg_any = Any {
        type_url: MsgMint::TYPE_URL.to_string(),
        value: mint_msg_binary.to_vec(),
    };

    let exec_msg = MsgExec {
        grantee: env.contract.address.to_string(),
        msgs: vec![mint_msg_any],
    };

    REPLY_STATE.save(
        deps.storage,
        AUTHZ_EXEC_MINT_MSG_ID,
        &MintReplyState {
            claimer_addr,
            amount,
            denom: full_denom,
        },
    )?;

    deps.api.debug("claim end");

    Ok(Response::new()
        .add_attribute("action", "claim")
        .add_submessage(SubMsg::reply_on_success(exec_msg, AUTHZ_EXEC_MINT_MSG_ID)))
}

pub fn verify_proof(
    merkle_root: &String,
    proof_str: &str,
    to_verify: String,
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

    if !proof.verify(&to_verify, &root_hash) {
        return Err(ContractError::FailedVerifyProof {});
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_ROOT test merkel root that was generated from "testdata/uosmo_only.csv" using merkle-drop-cli
    const TEST_ROOT: &str = "Nz54SQtyBVHwsmEqNI//mxFgiq8MRD7sS92IGkhgMvo=";
    const TEST_ROOT2_ADDR_AMOUNT: &str = "1V0YcwzXWtB+iuOTob6juiNliUmB278xZIKMnzwjqOU=";

    const VALID_PROOF_STR: &str = "[{\"is_left_sibling\":true,\"hash\":[77,122,52,81,83,57,77,56,52,118,78,84,89,115,86,76,120,80,121,50,109,89,49,111,107,85,111,68,111,108,97,118,105,81,119,71,114,54,111,49,51,107,99,61]},{\"is_left_sibling\":true,\"hash\":[113,53,102,108,80,98,70,114,73,121,100,108,105,97,121,52,85,73,57,100,107,111,106,71,82,116,49,57,90,71,121,105,51,111,55,104,103,112,87,103,73,117,48,61]},{\"is_left_sibling\":true,\"hash\":[114,87,52,69,82,120,75,113,110,53,102,77,69,65,114,57,56,118,90,80,116,54,119,108,67,71,49,69,80,121,99,57,51,54,108,48,112,100,86,97,100,120,48,61]},{\"is_left_sibling\":false,\"hash\":[66,86,43,54,113,43,104,43,100,99,115,116,50,83,66,52,122,68,111,100,118,50,98,90,84,112,108,105,105,65,104,87,50,82,75,121,112,67,72,51,81,66,85,61]},{\"is_left_sibling\":false,\"hash\":[86,100,75,79,107,112,115,120,78,73,103,43,50,111,102,47,57,76,84,111,122,51,107,84,102,53,112,90,113,81,108,108,112,86,84,86,82,55,108,77,103,83,119,61]}]";
    const VALID_PROOF_STR2_ADDR_AMOUNT: &str = "[{\"is_left_sibling\":true,\"hash\":[89,79,106,114,49,69,77,102,68,119,114,48,69,84,73,103,82,71,97,108,48,79,108,53,105,56,82,103,111,57,85,51,76,70,82,90,115,66,97,78,89,51,73,61]},{\"is_left_sibling\":false,\"hash\":[80,54,110,55,43,55,72,72,111,52,109,104,79,104,102,105,108,83,43,118,87,54,88,85,88,113,48,115,105,99,83,116,116,52,112,54,119,114,68,48,113,47,73,61]},{\"is_left_sibling\":true,\"hash\":[79,79,110,66,86,100,72,56,121,84,70,57,115,78,65,56,80,85,81,97,111,71,89,119,81,89,87,83,109,71,116,89,56,79,118,85,118,98,73,83,122,74,77,61]},{\"is_left_sibling\":false,\"hash\":[102,65,68,121,57,69,49,118,56,70,78,78,81,53,109,47,50,120,78,55,103,110,119,89,78,82,104,80,83,53,69,105,79,53,115,79,77,43,118,106,50,98,56,61]}]";

    const INVALID_PROOF_STR: &str = "[{\"is_left_sibling\":true,\"hash\":[78,122,52,81,83,57,77,56,52,118,78,84,89,115,86,76,120,80,121,50,109,89,49,111,107,85,111,68,111,108,97,118,105,81,119,71,114,54,111,49,51,107,99,61]},{\"is_left_sibling\":true,\"hash\":[113,53,102,108,80,98,70,114,73,121,100,108,105,97,121,52,85,73,57,100,107,111,106,71,82,116,49,57,90,71,121,105,51,111,55,104,103,112,87,103,73,117,48,61]},{\"is_left_sibling\":true,\"hash\":[114,87,52,69,82,120,75,113,110,53,102,77,69,65,114,57,56,118,90,80,116,54,119,108,67,71,49,69,80,121,99,57,51,54,108,48,112,100,86,97,100,120,48,61]},{\"is_left_sibling\":false,\"hash\":[66,86,43,54,113,43,104,43,100,99,115,116,50,83,66,52,122,68,111,100,118,50,98,90,84,112,108,105,105,65,104,87,50,82,75,121,112,67,72,51,81,66,85,61]},{\"is_left_sibling\":false,\"hash\":[86,100,75,79,107,112,115,120,78,73,103,43,50,111,102,47,57,76,84,111,122,51,107,84,102,53,112,90,113,81,108,108,112,86,84,86,82,55,108,77,103,83,119,61]}]";

    const TO_VERIFY_VALID: &str = "osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo";
    const TO_VERIFY_VALID2_ADDR_AMOUNT: &str = "osmo1hqslwuc8ukaaaxfmahgnquyqx3w0tmrluwxmxj1421901";

    #[test]
    fn verify_proof_success() {
        verify_proof(
            &String::from(TEST_ROOT),
            &String::from(VALID_PROOF_STR),
            String::from(TO_VERIFY_VALID),
        )
        .unwrap();
    }

    #[test]
    fn verify_proof_amount_addr_success() {
        verify_proof(
            &String::from(TEST_ROOT2_ADDR_AMOUNT),
            &String::from(VALID_PROOF_STR2_ADDR_AMOUNT),
            String::from(TO_VERIFY_VALID2_ADDR_AMOUNT),
        )
        .unwrap();
    }

    #[test]
    fn verify_proof_invalid_root_error() {
        verify_proof(
            &String::from("this is garbage"),
            &String::from(VALID_PROOF_STR),
            String::from(TO_VERIFY_VALID),
        )
        .unwrap_err();
    }

    #[test]
    fn verify_proof_invalid_proof_error() {
        verify_proof(
            &String::from(TEST_ROOT),
            &String::from(INVALID_PROOF_STR),
            String::from(TO_VERIFY_VALID),
        )
        .unwrap_err();
    }
}
