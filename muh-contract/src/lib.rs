#![crate_name = "muh_contract"]

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError
};

use cw2::set_contract_version;
use cw_storage_plus::Map;

use serde::{Deserialize, Serialize};
use schemars::JsonSchema; 

const CONTRACT_NAME: &str = "crates.io:secure-update";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Update {
    key: String,
    checksum: String,
    cid: String,
    update_version: String,
    update_success: Vec<String>,
    update_fail: Vec<String>,
}

pub static ADMIN: &str = "ADMIN";
pub static MODEL_A: &str = "MODEL_A";
pub static MODEL_B: &str = "MODEL_B";
pub static MODEL_C: &str = "MODEL_C";

pub const UPDATES: Map<&str, Update> = Map::new("updates");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Define roles and the admin on instantiation
    let admin_addr = info.sender.clone();
    // Help: Normally we would set up role management here. This can be adapted based on further details
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", admin_addr.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddUpdate { model, update } => add_update(deps, info, model, update),
        // More handlers here
    }
}

fn add_update(deps: DepsMut, info: MessageInfo, model: String, update: Update) -> StdResult<Response> {
    // Check if sender is admin
    if info.sender != ADMIN {
        return Err(StdError::generic_err("Unauthorized Access"));
    }

    UPDATES.save(deps.storage, &model, &update)?;

    Ok(Response::new().add_attribute("action", "add_update"))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUpdate { model } => to_binary(&query_update(deps, model)?),
    }
}

fn query_update(deps: Deps, model: String) -> StdResult<Update> {
    UPDATES.load(deps.storage, &model)
}

// Define the different message types for execute and query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    AddUpdate {
        model: String,
        update: Update,
    },
    // More execute message variants
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetUpdate {
        model: String,
    },
    // More query message variants
}

