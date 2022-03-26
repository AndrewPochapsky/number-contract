use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

pub const NUMBERS: Map<&str, Uint128> = Map::new("numbers");

pub const OWNER: Item<String> = Item::new("owner");
