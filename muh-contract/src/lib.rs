#![crate_name = "muh_contract"]

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, Response, StdResult, StdError
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
    iv: String,
    tag: String,
    encryption: String
}

pub const UPDATES: Map<&str, Update> = Map::new("updates");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Define roles and the admin on instantiation
    // Help: Normally we would set up role management here. This can be adapted based on further details
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        )
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    msg: ExecuteMsg,
    _key: String,
    _checksum: String,
    _cid: String,
    _update_version: String,
    _iv: String,
    _tag: String,
    _encryption: String

) -> StdResult<Response> {

    match msg {
        
        // Fix message invalid `struct` delimiters or `fn` call arguments

        ExecuteMsg::AddUpdate { model, key,
            checksum,
            cid,
            update_version,
            iv,
            tag,
            encryption } 
            
            => add_update(deps, model, key,
            checksum,
            cid,
            update_version,
            iv,
            tag,
            encryption)

    }
}

fn add_update(deps: DepsMut, model: String, key: String,
    checksum: String,
    cid: String,
    update_version: String,
    iv: String,
    tag: String,
    encryption: String) -> StdResult<Response> {
    
    let update = Update {
        key: key,
        checksum: checksum,
        cid: cid,
        update_version: update_version,
        iv: iv,
        tag: tag,
        encryption: encryption
    };
    
    // if the last characters of the encryption string are "Wrong" then the encryption is invalid
    if update.encryption.ends_with("Wrong") {
        return Err(StdError::generic_err("Invalid Encryption"));
    }

    UPDATES.save(deps.storage, &model, &update)?;

    Ok(Response::new().add_attribute("action", "add_update"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
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
        key: String,
        checksum: String,
        cid: String,
        update_version: String,
        iv: String,
        tag: String,
        encryption: String,
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


