use cosmwasm_std::to_binary;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Order};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;

const CONTRACT_NAME: &str = "crates.io:ufc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = msg.admin.unwrap_or_else(|| info.sender.to_string());

    match deps.api.addr_validate(&admin) {
        Ok(x) => {
            let config = Config { admin: x };
            CONFIG.save(deps.storage, &config)?;
        }
        Err(_) => return Err(ContractError::CerealError {}),
    }

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddFight {
            fight_id,
            fighters,
            pick,
        } => execute_create_fight(deps, env, info, fight_id, fighters, pick),
    }
}

pub fn execute_create_fight(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    fight_id: String,
    fighters: String,
    pick: Pick,
) -> Result<Response, ContractError> {
    match CONFIG.load(deps.storage) {
        Ok(x) => {
            let configx: Config = x;
            if configx.admin != info.sender {
                return Err(ContractError::Unauthorized {});
            };
        }
        Err(_) => return Err(ContractError::UndefinedError {}),
    };

    match HISTORY.save(deps.storage, (fight_id, fighters.clone()), &pick) {
        Ok(_) => {
            return Ok(Response::new()
                .add_attribute("action", "create a fight")
                .add_attribute("fighters", fighters))
        }
        Err(_) => return Err(ContractError::AddFightError {}),
    };
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Get all picks for a given fight_id <Given 1st value in key, return all instances>
        QueryMsg::GetCardPicks { fight_id } => query_card_picks(deps, fight_id),

        // Get a specific picks odds given both keys <Given both values in key, return the Pick>
        QueryMsg::GetPick { fight_id, fighters } => query_pick(deps, fight_id, fighters),
    }
}

pub fn query_card_picks(deps: Deps, fight_id: String) -> StdResult<Binary> {
    let history: StdResult<Vec<_>> = HISTORY
        .prefix(fight_id)
        .range(deps.storage, None, None, Order::Descending)
        .take(15)
        .collect();

    match history {
        Err(error) => Err(error),
        Ok(x) => to_binary(&GetCardResponse { picks: x }),
    }



    
}

pub fn query_pick(deps: Deps, fight_id: String, fighters: String) -> StdResult<Binary> {
    match HISTORY.may_load(deps.storage, (fight_id, fighters)) {
        Ok(s) => match s {
            Some(pick) => to_binary(&GetPickResponse { pick: Some(pick) }),
            None => to_binary(&GetPickResponse { pick: None }),
        },
        Err(err) => Err(err),
    }
}
