use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Uint64};
use cosmwasm_std::testing::MockApi;
use cw_storage_plus::{Item, Map};
use cw_utils::Scheduled;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {}

pub const CONFIG: Item<Config> = Item::new("admin");

pub const LOCK_BOX_SEQ: Item<Uint64> = Item::new("lockbox_seq");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Lockbox {
    pub id: Uint64,
    /// Owner is the owner of lockbox
    pub owner: Addr,
    pub claims: Vec<Claim>,
    pub expiration: Scheduled,
    pub total_amount: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Claim {
    pub addr: String,
    pub amount: Uint128
}

pub const LOCKBOXES: Map<u64, Lockbox> = Map::new("lock_boxes");
