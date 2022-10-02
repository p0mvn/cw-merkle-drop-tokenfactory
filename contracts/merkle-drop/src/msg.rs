use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub merkle_root: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetSubDenom {
        subdenom: String,
    },
    Claim {
        proof: String,
        amount: Uint128,
        claimer_addr: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetRootResponse)]
    GetRoot {},
    #[returns(GetSubDenomResponse)]
    GetSubdenom {},
}

#[cw_serde]
pub struct GetRootResponse {
    pub root: String,
}

#[cw_serde]
pub struct GetSubDenomResponse {
    pub subdenom: String,
}
