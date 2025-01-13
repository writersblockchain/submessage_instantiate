use crate::error::ContractError;
use anybuf::Anybuf;
use cosmwasm_std::{
    entry_point, to_binary, to_vec, Binary, ContractResult, DepsMut, Env, MessageInfo, Reply,
    Response, StdError, StdResult, SubMsg, SubMsgResult, SystemResult, WasmMsg,
};
use secret_toolkit::utils::HandleCallback;

use crate::msg::{ExecuteMsg, InstantiateCountMsg, InstantiateMsg};
use crate::state::{EXECUTE_INCREMENT_REPLY_ID, EXECUTE_INSTANTIATE_REPLY_ID};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment { contract } => try_increment(deps, env, contract),
        ExecuteMsg::Instantiate { code_id, code_hash } => {
            try_instantiate(deps, env, code_id, code_hash)
        }
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        EXECUTE_INCREMENT_REPLY_ID => handle_increment_reply(deps, msg),
        EXECUTE_INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        id => Err(ContractError::UnexpectedReplyId { id }),
    }
}

fn handle_increment_reply(_deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    match msg.result {
        SubMsgResult::Ok(_) => Ok(Response::new().add_attribute("action", "increment")),

        SubMsgResult::Err(e) => Err(ContractError::CustomError { val: e }),
    }
}

fn handle_instantiate_reply(_deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    match msg.result {
        SubMsgResult::Ok(_) => Ok(Response::new().add_attribute("action", "instantiate")),

        SubMsgResult::Err(e) => Err(ContractError::CustomError { val: e }),
    }
}

pub fn try_increment(deps: DepsMut, _env: Env, contract: String) -> StdResult<Response> {
    // Create the execution message for increment
    let exec_msg = ExecuteMsg::Increment {
        contract: contract.clone(),
    };

    // Retrieve the contract code hash
    let code_hash = get_contract_code_hash(deps, contract.clone())?;

    // Create the submessage
    let submsg = SubMsg::reply_always(
        exec_msg.to_cosmos_msg(code_hash, contract.clone(), None)?,
        EXECUTE_INCREMENT_REPLY_ID,
    );

    // Return the response with the submessage
    Ok(Response::new().add_submessage(submsg))
}

pub fn try_instantiate(
    _deps: DepsMut,
    _env: Env,
    code_id: u64,
    code_hash: String,
) -> StdResult<Response> {
    let msg = to_binary(&InstantiateCountMsg { count: 0 })?;

    let submsg = SubMsg::reply_always(
        WasmMsg::Instantiate {
            code_id: code_id,
            code_hash: code_hash,
            label: "my-contract".to_string(),
            funds: vec![],
            admin: None,
            msg,
        },
        EXECUTE_INSTANTIATE_REPLY_ID,
    );

    Ok(Response::new().add_submessage(submsg))
}

fn get_contract_code_hash(deps: DepsMut, contract_address: String) -> StdResult<String> {
    let code_hash_query: cosmwasm_std::QueryRequest<cosmwasm_std::Empty> =
        cosmwasm_std::QueryRequest::Stargate {
            path: "/secret.compute.v1beta1.Query/CodeHashByContractAddress".into(),
            data: Binary(Anybuf::new().append_string(1, contract_address).into_vec()),
        };

    let raw = to_vec(&code_hash_query).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;

    let code_hash = match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }?;

    // Remove the "\n@" if it exists at the start of the code_hash
    let mut code_hash_str = String::from_utf8(code_hash.to_vec())
        .map_err(|err| StdError::generic_err(format!("Invalid UTF-8 sequence: {}", err)))?;

    if code_hash_str.starts_with("\n@") {
        code_hash_str = code_hash_str.trim_start_matches("\n@").to_string();
    }

    Ok(code_hash_str)
}
