#![crate_name = "muh_contract"]

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use cw2::set_contract_version;
use cw_storage_plus::Map;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CONTRACT_NAME: &str = "crates.io:secure-update";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Update {
    key: String,
    checksum: String,
    cid: String,
    update_version: String,
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
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddUpdate { model, update } => add_update(deps, info, model, update),
        // More handlers here
    }
}

fn add_update(
    deps: DepsMut,
    info: MessageInfo,
    model: String,
    update: Update,
) -> StdResult<Response> {
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
    AddUpdate { model: String, update: Update },
    // More execute message variants
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetUpdate { model: String },
    // More query message variants
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        // Call the instantiate function
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Verify the expected result
        assert_eq!(0, res.messages.len());
        assert_eq!(res.attributes[0], attr("method", "instantiate"));
        assert_eq!(res.attributes[1], attr("admin", "creator"));
    }

    #[test]
    fn add_authorized_update() {
        let mut deps = mock_dependencies();
        let admin_info = mock_info(ADMIN, &[]);
        let instantiate_msg = InstantiateMsg {};
        let _ = instantiate(
            deps.as_mut(),
            mock_env(),
            admin_info.clone(),
            instantiate_msg,
        )
        .unwrap();

        // Update should work for modelA rather than modelB
        let update = Update {
            key: "muhKey".to_string(),
            checksum: "muhChecksum".to_string(),
            cid: "muhCid".to_string(),
            update_version: "v1.0".to_string(),
        };

        let execute_msg = ExecuteMsg::AddUpdate {
            model: MODEL_A.to_string(),
            update: update.clone(),
        };

        let res = execute(deps.as_mut(), mock_env(), admin_info, execute_msg).unwrap();
        assert_eq!(res.attributes[0], attr("action", "add_update"));

        let query_msg = QueryMsg::GetUpdate {
            model: MODEL_A.to_string(),
        };
        let bin = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let stored_update: Update = from_binary(&bin).unwrap();
        assert_eq!(stored_update, update);
    }

    #[test]
    fn unauthorized_access() {
        let mut deps = mock_dependencies();
        let info = mock_info("joe_shmoe", &[]);

        let update = Update {
            key: "key123".to_string(),
            checksum: "checksumabc".to_string(),
            cid: "cidXYZ".to_string(),
            update_version: "v1.0".to_string(),
        };

        let execute_msg = ExecuteMsg::AddUpdate {
            model: MODEL_A.to_string(),
            update: update.clone(),
        };

        // Try to execute with non-admin user
        let res = execute(deps.as_mut(), mock_env(), info, execute_msg);

        // Expect an error due to unauthorized access
        assert!(res.is_err());
    }
}
