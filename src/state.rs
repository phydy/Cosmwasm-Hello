use std::fmt::Debug;

use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Hello {
    pub greeting: String,
}

pub const GREETING: Item<Hello> = Item::new("greeting");
