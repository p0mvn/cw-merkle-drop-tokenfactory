use std::path::PathBuf;

use cosmwasm_std::Coin;
use merkle_drop::msg::{InstantiateMsg, QueryMsg};
use osmosis_std::types::osmosis::tokenfactory;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgChangeAdmin, MsgChangeAdminResponse, MsgCreateDenom, MsgCreateDenomResponse,
    QueryDenomAuthorityMetadataRequest, QueryDenomAuthorityMetadataResponse,
};
use osmosis_testing::{Account, ExecuteResponse, OsmosisTestApp, Runner, SigningAccount};
use osmosis_testing::{Gamm, Module, Wasm};

const TEST_ROOT: &str = "1V0YcwzXWtB+iuOTob6juiNliUmB278xZIKMnzwjqOU=";

pub const VALID_SUBDENOM: &str = "subdenom";

pub struct TestEnv {
    pub app: OsmosisTestApp,
    pub contract_address: String,
    pub owner: SigningAccount,
}
impl TestEnv {
    pub fn new() -> Self {
        let app = OsmosisTestApp::new();
        let wasm = Wasm::new(&app);

        // setup owner account
        let initial_balance = [
            Coin::new(1_000_000_000_000, "uosmo"),
            Coin::new(1_000_000_000_000, "uion"),
            Coin::new(1_000_000_000_000, "stake"),
        ];
        let owner = app.init_account(&initial_balance).unwrap();

        // Create denom
        let create_denom_msg = tokenfactory::v1beta1::MsgCreateDenom {
            sender: owner.address(),
            subdenom: String::from(VALID_SUBDENOM),
        };
        let _res: ExecuteResponse<MsgCreateDenomResponse> = app
            .execute(create_denom_msg, MsgCreateDenom::TYPE_URL, &owner)
            .unwrap();

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

        let full_denom = format!("factory/{}/{}", owner.address(), VALID_SUBDENOM);

        // Simulate authz Grant by changing admin to contract address
        let change_admin_msg = tokenfactory::v1beta1::MsgChangeAdmin {
            sender: owner.address(),
            denom: full_denom.clone(),
            new_admin: contract_address.clone(),
        };

        let _res: ExecuteResponse<MsgChangeAdminResponse> = app
            .execute(change_admin_msg, MsgChangeAdmin::TYPE_URL, &owner)
            .unwrap();

        let admin_query = QueryDenomAuthorityMetadataRequest { denom: full_denom };

        let admin = app
            .query::<QueryDenomAuthorityMetadataRequest, QueryDenomAuthorityMetadataResponse>(
                "/osmosis.tokenfactory.v1beta1.Query/DenomAuthorityMetadata",
                &admin_query,
            )
            .unwrap()
            .authority_metadata
            .unwrap()
            .admin;

        println!("setup admin {}", admin);
        println!("contract address {}", contract_address);
        if !admin.eq(&contract_address) {
            panic!(
                "{}",
                format!(
                    "admin from response {} is not equal to contract address {}",
                    admin, contract_address
                )
            );
        }

        TestEnv {
            app,
            contract_address,
            owner,
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
