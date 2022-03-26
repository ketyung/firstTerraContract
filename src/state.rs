use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
    pub message : String, 
    pub updated : Timestamp, 
}

pub const STATE: Item<State> = Item::new("state");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Member {

    pub name : String,

    pub age : i8,

    pub date_joined : Timestamp, 
}

pub const MEMBERS : Map<&str, Member> = Map::new("members");

