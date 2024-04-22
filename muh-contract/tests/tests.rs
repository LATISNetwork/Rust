use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, Addr, to_binary};
use cosmwasm_vm::testing::{mock_instance, instantiate, execute, query};

use muh_contract::{ExecuteMsg, QueryMsg, Update, ADMIN, MODEL_A};


#[test]
fn test_instantiate() {

    static WASM_BINARY: &[u8] = include_bytes!("home/matthewc458/muh-contract/target/wasm32-unknown-unknown/release/muh_contract.wasm");
    let mut deps = mock_instance(WASM_BINARY, &[]);
// mock_dependencies();
    let env = mock_env();
    let info = mock_info("creator", &[]);

    let res = instantiate(&mut deps, env, info, InstantiateMsg {}).unwrap();
   
    assert_eq!(0, res.messages.len());
}

#[test]
fn test_add_update() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let admin_info = mock_info(ADMIN, &[]);

    let update = Update {
        key: "update_key".to_string(),
        checksum: "checksum".to_string(),
        cid: "cid".to_string(),
        update_version: "1.0.0".to_string(),
        update_success: vec![],
        update_fail: vec![],
    };

    let msg = ExecuteMsg::AddUpdate {
        model: MODEL_A.to_string(),
        update: update.clone(),
    };

    let res = execute(deps.as_mut(), env.clone(), admin_info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    let query_msg = QueryMsg::GetUpdate {
        model: MODEL_A.to_string(),
    };
    let res_bin = query(deps.as_ref(), env, query_msg).unwrap();
    let loaded_update: Update = from_binary(&res_bin).unwrap();
    assert_eq!(update, loaded_update);
}

fn main() {


}

