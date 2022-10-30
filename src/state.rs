// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint64};
use cw_storage_plus::Item;

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct State {
//     pub count: i32,
//     pub owner: Addr,
// }
// pub const STATE: Item<State> = Item::new("state");
// little change to delete

pub const PING_COUNT: Item<Uint64> = Item::new("ping_count_key");

