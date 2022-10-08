mod test_env;
use cosmwasm_std::{Coin, Uint128};
use merkle_drop::msg::ExecuteMsg;
use osmosis_testing::{
    cosmrs::proto::cosmos::bank::v1beta1::{QueryBalanceRequest, QueryBalanceResponse},
    Module, Runner, Wasm,
};
use test_env::*;

const VALID_PROOF_STR: &str = "[{\"is_left_sibling\":true,\"hash\":[89,79,106,114,49,69,77,102,68,119,114,48,69,84,73,103,82,71,97,108,48,79,108,53,105,56,82,103,111,57,85,51,76,70,82,90,115,66,97,78,89,51,73,61]},{\"is_left_sibling\":false,\"hash\":[80,54,110,55,43,55,72,72,111,52,109,104,79,104,102,105,108,83,43,118,87,54,88,85,88,113,48,115,105,99,83,116,116,52,112,54,119,114,68,48,113,47,73,61]},{\"is_left_sibling\":true,\"hash\":[79,79,110,66,86,100,72,56,121,84,70,57,115,78,65,56,80,85,81,97,111,71,89,119,81,89,87,83,109,71,116,89,56,79,118,85,118,98,73,83,122,74,77,61]},{\"is_left_sibling\":false,\"hash\":[102,65,68,121,57,69,49,118,56,70,78,78,81,53,109,47,50,120,78,55,103,110,119,89,78,82,104,80,83,53,69,105,79,53,115,79,77,43,118,106,50,98,56,61]}]";
const TO_VERIFY_VALID2_ADDR_AMOUNT: &str = "osmo1hqslwuc8ukaaaxfmahgnquyqx3w0tmrluwxmxj";

test_claim!(
    claim
    should succeed,

    proof: String::from(VALID_PROOF_STR),
    amount: Uint128::from(1421901 as u128)
);

// ======= helpers ========

#[macro_export]
macro_rules! test_claim {
    ($test_name:ident should succeed, proof: $proof:expr, amount: $amount:expr) => {
        #[test]
        fn $test_name() {
            test_claim_success_case($proof, $amount)
        }
    };
}

fn test_claim_success_case(proof: String, amount: Uint128) {
    let test_env = TestEnv::new();

    test_env.execute_msg_grant_mint();
    test_env.execute_msg_grant_bank_send();

    let TestEnv {
        app,
        contract_address,
        owner,
        full_denom,
    } = test_env;

    let set_subdenom_msg = ExecuteMsg::SetSubDenom {
        subdenom: String::from(VALID_SUBDENOM),
    };
    // setup denum from owner address
    let wasm = Wasm::new(&app);
    let _res = wasm.execute(&contract_address, &set_subdenom_msg, &[], &owner);

    // claim from a new address
    let initial_balance = [Coin::new(1_000_000_000_000, "uosmo")];
    let claim_sender = app.init_account(&initial_balance).unwrap();

    let claimer_addr = String::from(TO_VERIFY_VALID2_ADDR_AMOUNT);

    let msg = ExecuteMsg::Claim {
        proof: proof,
        amount: amount,
        claimer_addr: claimer_addr.clone(),
    };

    let wasm = Wasm::new(&app);
    let res = wasm.execute(&contract_address, &msg, &[], &claim_sender);

    // check if execution succeeded
    assert!(res.is_ok(), "{:?}", res.unwrap_err());

    let balances_query = QueryBalanceRequest {
        denom: full_denom.clone(),
        address: claimer_addr.clone(),
    };

    let balance = app
        .query::<QueryBalanceRequest, QueryBalanceResponse>(
            "/cosmos.bank.v1beta1.Query/Balance",
            &balances_query,
        )
        .unwrap()
        .balance
        .unwrap();

    let actual_amount = Uint128::from(balance.amount.parse::<u128>().unwrap());

    assert_eq!(full_denom, balance.denom);
    assert_eq!(amount, actual_amount);
}
