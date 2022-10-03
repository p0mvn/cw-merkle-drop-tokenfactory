use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub merkle_root: String,
    pub owner: Addr,
}

#[cw_serde]
pub struct MintReplyState {
    pub claimer_addr: String,
    pub amount: Uint128,
    pub denom: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const SUBDENOM: Item<String> = Item::new("subdenom");

pub const CLAIMED_ADDRESSES: Map<&str, bool> = Map::new("claimed");

// MINT_REPLY_STATE persists data from tf mint message creation until the reply receipt.
pub const MINT_REPLY_STATE: Map<u64, MintReplyState> = Map::new("mint_reply_state");
