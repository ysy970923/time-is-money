use cosmwasm_std::{
    from_binary, to_binary, Addr, CosmosMsg, DepsMut, Env,
    MessageInfo, Response, Uint128, WasmMsg,
};
use cw1155::Cw1155ExecuteMsg;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::msg::{BlunaExecuteMsg, Cw20HookMsg};
use crate::state::{config_read, state_read, state_store, Config, State};

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let config: Config = config_read(deps.storage).load()?;
    // cw 20 contract only has authorization
    if config.bluna_token != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(ContractError::Unauthorized {});
    }

    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::UnbondBluna {}) => {
            let api = deps.api;
            unbond_bluna(
                deps,
                env,
                api.addr_validate(&cw20_msg.sender)?,
                cw20_msg.amount,
            )
        }
        _ => Err(ContractError::DataShouldBeGiven {}),
    }
}

pub fn unbond_bluna(
    deps: DepsMut,
    env: Env,
    sender: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config: Config = config_read(deps.storage).load()?;
    let mut state: State = state_store(deps.storage).load()?;
    state.unbonded_amount += amount;
    let token_id = env.block.height.to_string();
    let unbond_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.bluna_token)?.to_string(),
        funds: vec![],
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps.api.addr_humanize(&config.bluna_token)?.to_string(),
            amount: amount,
            msg: to_binary(&BlunaExecuteMsg::Unbond {})?,
        })?,
    });
    // enough fund
    let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.cw1155_token)?.to_string(),
        funds: vec![],
        msg: to_binary(&Cw1155ExecuteMsg::Mint {
            to: sender.to_string(),
            token_id: token_id,
            value: amount,
            msg: None,
        })?,
    });
    let messages: Vec<CosmosMsg> = vec![unbond_msg, mint_msg];

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("action", "unbond_bluna"),
        ("block_height", &token_id),
        ("sender", sender.as_str()),
    ]))
}

pub fn execute_withdraw_unbonded(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let sender_human = info.sender;
    let config: Config = config_read(deps.storage).load()?;
    let withdraw_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.bluna_token)?.to_string(),
        funds: vec![],
        msg: to_binary(&BlunaExecuteMsg::WithdrawUnbonded {})?,
    });
}
