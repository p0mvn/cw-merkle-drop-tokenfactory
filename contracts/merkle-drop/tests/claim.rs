mod test_env;
use cosmwasm_std::{Coin, Uint128};
use merkle_drop::msg::{ExecuteMsg, QueryMsg};
use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;
use osmosis_testing::{Module, RunnerError, Wasm};
use test_env::*;

const VALID_PROOF_STR: &str = "[{\"is_left_sibling\":true,\"hash\":[77,122,52,81,83,57,77,56,52,118,78,84,89,115,86,76,120,80,121,50,109,89,49,111,107,85,111,68,111,108,97,118,105,81,119,71,114,54,111,49,51,107,99,61]},{\"is_left_sibling\":true,\"hash\":[113,53,102,108,80,98,70,114,73,121,100,108,105,97,121,52,85,73,57,100,107,111,106,71,82,116,49,57,90,71,121,105,51,111,55,104,103,112,87,103,73,117,48,61]},{\"is_left_sibling\":true,\"hash\":[114,87,52,69,82,120,75,113,110,53,102,77,69,65,114,57,56,118,90,80,116,54,119,108,67,71,49,69,80,121,99,57,51,54,108,48,112,100,86,97,100,120,48,61]},{\"is_left_sibling\":false,\"hash\":[66,86,43,54,113,43,104,43,100,99,115,116,50,83,66,52,122,68,111,100,118,50,98,90,84,112,108,105,105,65,104,87,50,82,75,121,112,67,72,51,81,66,85,61]},{\"is_left_sibling\":false,\"hash\":[86,100,75,79,107,112,115,120,78,73,103,43,50,111,102,47,57,76,84,111,122,51,107,84,102,53,112,90,113,81,108,108,112,86,84,86,82,55,108,77,103,83,119,61]}]";

test_claim!(
    claim
    should succeed,

    proof: String::from(VALID_PROOF_STR),
    amount: Coin { denom: String::from("uosmo"), amount: Uint128::from(503 as u128) }
);

// test_set_route!(
//     set_initial_route_by_non_owner
//     should failed_with "Unauthorized: execute wasm contract failed",

//     sender = NonOwner,
//     msg = ExecuteMsg::SetRoute {
//         input_denom: "uosmo".to_string(),
//         output_denom: "uion".to_string(),
//         pool_route: vec![SwapAmountInRoute {
//             pool_id: 1,
//             token_out_denom: "uion".to_string(),
//         }],
//     }
// );

// ======= helpers ========

#[macro_export]
macro_rules! test_claim {
    ($test_name:ident should succeed, proof: $proof:expr, amount: $amount:expr) => {
        #[test]
        fn $test_name() {
            test_set_route_success_case($proof, $amount)
        }
    };
}

fn test_set_route_success_case(proof: String, amount: Coin) {
    let TestEnv {
        app,
        contract_address,
        owner,
        valid_sender,
    } = TestEnv::new();

    let msg = ExecuteMsg::Claim {
        proof: proof,
        amount: amount,
    };

    let wasm = Wasm::new(&app);
    let res = wasm.execute(&contract_address, &msg, &[], &valid_sender);

    // check if execution succeeded
    assert!(res.is_ok(), "{:?}", res.unwrap_err());

    // check if previously set route can be queried correctly
    match msg {
        ExecuteMsg::Claim { proof, amount, .. } => {
            // validate that minted and sent to claimer
        }
        _ => {
            panic!("ExecuteMsg must be `SetRoute`");
        }
    }
}

// fn test_set_route_failed_case(sender: Sender, msg: ExecuteMsg, expected_error: &str) {
//     let TestEnv {
//         app,
//         contract_address,
//         owner,
//     } = TestEnv::new();
//     let wasm = Wasm::new(&app);

//     let sender = match sender {
//         Sender::Owner => owner,
//         Sender::NonOwner => {
//             let initial_balance = [
//                 Coin::new(1_000_000_000_000, "uosmo"),
//                 Coin::new(1_000_000_000_000, "uion"),
//                 Coin::new(1_000_000_000_000, "stake"),
//             ];
//             app.init_account(&initial_balance).unwrap()
//         }
//     };

//     let res = wasm.execute::<ExecuteMsg>(&contract_address, &msg, &[], &sender);
//     let err = res.unwrap_err();

//     // assert on error message
//     if let RunnerError::ExecuteError { msg } = &err {
//         let expected_err = &format!(
//             "failed to execute message; message index: 0: {}",
//             expected_error
//         );
//         assert_eq!(msg, expected_err);
//     } else {
//         panic!("unexpected error: {:?}", err);
//     }
// }
