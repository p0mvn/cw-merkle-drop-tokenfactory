use cosmwasm_std::{Coin};
use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub merkle_root: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetDenom {
        subdenom: String,
    },
    Claim {
        proof: String,
        amount: Coin,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetRootResponse)]
    GetRoot {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetRootResponse {
    pub root: String,
}
