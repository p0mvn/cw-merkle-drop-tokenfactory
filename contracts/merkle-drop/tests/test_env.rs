use std::path::PathBuf;

use cosmwasm_std::Coin;
use osmosis_std::types::osmosis::tokenfactory;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenomResponse, MsgCreateDenom};
use osmosis_testing::{Account, OsmosisTestApp, SigningAccount, Runner, ExecuteResponse};
use osmosis_testing::{Gamm, Module, Wasm};
use merkle_drop::msg::InstantiateMsg;

const TEST_ROOT: &str = "Nz54SQtyBVHwsmEqNI//mxFgiq8MRD7sS92IGkhgMvo=";

const VALID_SUBDENOM: &str = "subdenom";

pub struct TestEnv {
    pub app: OsmosisTestApp,
    pub contract_address: String,
    pub owner: SigningAccount,
    pub valid_sender: SigningAccount,
}
impl TestEnv {
    pub fn new() -> Self {
        let app = OsmosisTestApp::new();
        let gamm = Gamm::new(&app);
        let wasm = Wasm::new(&app);

        // setup owner account
        let initial_balance = [
            Coin::new(1_000_000_000_000, "uosmo"),
            Coin::new(1_000_000_000_000, "uion"),
            Coin::new(1_000_000_000_000, "stake"),
        ];
        let owner = app.init_account(&initial_balance).unwrap();

        let valid_sender = app.init_account(&initial_balance).unwrap();

        let create_denom_msg = tokenfactory::v1beta1::MsgCreateDenom{
            sender: valid_sender.address(),
            subdenom: String::from(VALID_SUBDENOM),
        };

        let _res: ExecuteResponse<MsgCreateDenomResponse> =
            app.execute(create_denom_msg, MsgCreateDenom::TYPE_URL, &valid_sender).unwrap();

        let code_id = wasm
            .store_code(&get_wasm(), None, &owner)
            .unwrap()
            .data
            .code_id;

        let contract_address = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    merkle_root: String::from(TEST_ROOT),
                },
                Some(&owner.address()),
                None,
                &[],
                &owner,
            )
            .unwrap()
            .data
            .address;

        TestEnv {
            app,
            contract_address,
            owner,
            valid_sender
        }
    }
}

fn get_wasm() -> Vec<u8> {
    let wasm_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("merkle_drop.wasm");
    std::fs::read(wasm_path).unwrap()
}
