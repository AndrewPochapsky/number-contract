use cosmwasm_std::Uint128;
use cw_storage_plus::Map;

pub const NUMBERS: Map<&str, Uint128> = Map::new("numbers");
