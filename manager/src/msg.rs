use crate::state::BLOCK_SIZE;
use schemars::JsonSchema;
use secret_toolkit::utils::HandleCallback;
use secret_toolkit::utils::InitCallback;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateCountMsg {
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment { contract: String },
    Instantiate { code_id: u64, code_hash: String },
}

impl InitCallback for ExecuteMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

impl HandleCallback for ExecuteMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}
