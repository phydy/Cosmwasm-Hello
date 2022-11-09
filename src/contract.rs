#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Hello, GREETING};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ixo-hello-world";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    //set the contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    //get the message
    let greeting = msg;
    //save the message item from the instantiation message
    GREETING.save(deps.storage, &greeting.first_message)?;
    //the result returned
    Ok(Response::new().add_attribute("instantiation", "first mesage"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ChangeGreeting { new_greeting } => {
            update_message(deps, env, info, new_greeting)
        }
    }
}

fn update_message(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    to_update: String, // supply a string of the greeting
) -> Result<Response, ContractError> {
    //save the message into state

    let data_to_update = Hello {
        greeting: to_update,
    };
    GREETING.save(deps.storage, &data_to_update)?;

    Ok(Response::new().add_attribute("message_update", data_to_update.greeting))
}

#[cfg_attr(not(feature = "library"), entry_point)]

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGreeting {} => to_binary(&get_current_greeting(deps)?),
    }
}

fn get_current_greeting(deps: Deps) -> StdResult<Hello> {
    let current_greeting = GREETING.load(deps.storage)?;
    Ok(Hello {
        greeting: current_greeting.greeting,
    })
}

#[cfg(test)]
mod tests {}
