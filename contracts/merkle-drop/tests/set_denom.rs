mod test_env;
use osmosis_testing::{Module, Wasm};
use merkle_drop::msg::{ExecuteMsg};
use test_env::*;

test_set_denom!(
    set_denom
    should succeed
);

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
    let TestEnv {
        app,
        contract_address,
        owner,
        valid_sender: _,
    } = TestEnv::new();

    let msg = ExecuteMsg::SetDenom { subdenom: String::from(VALID_SUBDENOM) };

    let wasm = Wasm::new(&app);
    let res = wasm.execute(&contract_address, &msg, &[], &owner);

    // check if execution succeeded
    assert!(res.is_ok(), "{:?}", res.unwrap_err());
}
