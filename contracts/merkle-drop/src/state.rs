use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub merkle_root: String,
    pub owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const CLAIM: Map<&str, bool> = Map::new("claimed");
