mod test_env;
use merkle_drop::msg::{ExecuteMsg, GetSubDenomResponse, QueryMsg};
use osmosis_std::shim::{Any, Timestamp};
use osmosis_std::types::cosmos::authz::v1beta1::{GenericAuthorization, Grant, MsgGrant};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;
use osmosis_testing::cosmrs::tx::MessageExt;
use osmosis_testing::{Account, ExecuteResponse, Module, Runner, Wasm};
use std::time::{SystemTime, UNIX_EPOCH};
use test_env::*;

const AIRDROP_SECONDS_DURATION: i64 = 60 * 60 * 5; // 5 hours from now
const AIRDROP_NANOS_DURATION: i32 = 0;

test_set_denom!(
    set_denom_valid_owner
    should succeed
);

// TODO: add edge case tests:
// - non-owner
// - contract owner but there is no denom created
// - authz grant is not issued - failure

// ======= helpers ========

#[macro_export]
macro_rules! test_set_denom {
    ($test_name:ident should succeed) => {
        #[test]
        fn $test_name() {
            test_set_denom_success_case()
        }
    };
}

fn test_set_denom_success_case() {
    let test_env = TestEnv::new();

    test_env.execute_msg_grant();

    let TestEnv {
        app,
        contract_address,
        owner,
    } = test_env;

    let subdenom = String::from(VALID_SUBDENOM);

    let set_subdenom_msg = ExecuteMsg::SetSubDenom {
        subdenom: subdenom.clone(),
    };

    let wasm = Wasm::new(&app);
    let res = wasm.execute(&contract_address, &set_subdenom_msg, &[], &owner);

    // check if execution succeeded
    assert!(res.is_ok(), "{:?}", res.unwrap_err());

    let get_subdenom_query = QueryMsg::GetSubdenom {};

    let q_res = wasm
        .query::<QueryMsg, GetSubDenomResponse>(&contract_address, &get_subdenom_query)
        .unwrap();

    assert_eq!(q_res.subdenom, subdenom.clone());
}
