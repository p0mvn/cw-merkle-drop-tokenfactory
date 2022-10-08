use std::path::PathBuf;

use cosmwasm_std::Coin;
use merkle_drop::msg::InstantiateMsg;
use osmosis_std::types::osmosis::tokenfactory;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgCreateDenomResponse};
use osmosis_testing::{cosmrs::tx::MessageExt, Module, Wasm};
use osmosis_testing::{Account, ExecuteResponse, OsmosisTestApp, Runner, SigningAccount};
use std::time::{SystemTime, UNIX_EPOCH};

use osmosis_std::{
    shim::{Any, Timestamp},
    types::{
        cosmos::authz::v1beta1::{GenericAuthorization, Grant, MsgGrant},
        osmosis::tokenfactory::v1beta1::MsgMint,
    },
};

const TEST_ROOT: &str = "1V0YcwzXWtB+iuOTob6juiNliUmB278xZIKMnzwjqOU=";

pub const VALID_SUBDENOM: &str = "subdenom";
const BANK_SEND_TYPE_URL: &str = "/cosmos.bank.v1beta1.MsgSend";

const AIRDROP_SECONDS_DURATION: i64 = 60 * 60 * 5; // 5 hours from now
const AIRDROP_NANOS_DURATION: i32 = 0;

pub struct TestEnv {
    pub app: OsmosisTestApp,
    pub contract_address: String,
    pub owner: SigningAccount,
    pub full_denom: String,
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

        TestEnv {
            app,
            contract_address,
            owner,
            full_denom,
        }
    }
}

pub trait Granter {
    fn execute_msg_grant_mint(&self);

    fn execute_msg_grant_bank_send(&self);

    fn execute_msg_grant(&self, authorization: Any, duration_since_unix_secs: i64);
}

impl Granter for TestEnv {
    fn execute_msg_grant_mint(&self) {
        let generic_mint_authorization = GenericAuthorization {
            msg: String::from(MsgMint::TYPE_URL),
        };

        let duration_since_unix_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.execute_msg_grant(
            Any {
                type_url: String::from(GenericAuthorization::TYPE_URL),
                value: generic_mint_authorization.to_bytes().unwrap(),
            },
            duration_since_unix_secs,
        )
    }

    fn execute_msg_grant_bank_send(&self) {
        // TODO: figure out serialization errors with the send authorization
        // let spend_authorization = SendAuthorization {
        //     spend_limit: vec![cosmos::base::v1beta1::Coin{ denom: self.full_denom.clone(), amount: String::from(MAX_SPEND_AUTHORIZATION_AMOUNT) }],
        // };
        let generic_send_authorization = GenericAuthorization {
            msg: String::from(BANK_SEND_TYPE_URL),
        };

        let duration_since_unix_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.execute_msg_grant(
            Any {
                type_url: String::from(GenericAuthorization::TYPE_URL),
                value: generic_send_authorization.to_bytes().unwrap(),
            },
            duration_since_unix_secs,
        )
    }

    fn execute_msg_grant(&self, authorization: Any, duration_since_unix_secs: i64) {
        // issue authz message
        let authz_grant_msg = MsgGrant {
            grantee: self.contract_address.clone(),
            granter: self.owner.address().clone(),
            grant: Some(Grant {
                authorization: Some(authorization),
                expiration: Some(Timestamp {
                    seconds: duration_since_unix_secs + AIRDROP_SECONDS_DURATION,
                    nanos: AIRDROP_NANOS_DURATION,
                }),
            }),
        };

        let _res: ExecuteResponse<MsgGrant> = self
            .app
            .execute(authz_grant_msg, MsgGrant::TYPE_URL, &self.owner)
            .unwrap();
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
