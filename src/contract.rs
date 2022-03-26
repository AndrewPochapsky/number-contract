use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{NUMBERS, OWNER},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.save(deps.storage, &info.sender.to_string())?;
    OWNER.remove(deps.storage);
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetNumber { number } => execute_set_number(deps, info, number),
        ExecuteMsg::DeleteNumber {} => execute_delete_number(deps, info),
    }
}

fn execute_set_number(
    deps: DepsMut,
    info: MessageInfo,
    number: Uint128,
) -> Result<Response, ContractError> {
    let sender = info.sender.to_string();
    NUMBERS.save(deps.storage, &sender, &number)?;
    Ok(Response::new()
        .add_attribute("action", "set_number")
        .add_attribute("number", number))
}

fn execute_delete_number(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let sender = info.sender.to_string();
    NUMBERS.remove(deps.storage, &sender);
    Ok(Response::new().add_attribute("action", "delete_number"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Number { address } => to_binary(&query_number(deps, address)?),
    }
}

fn query_number(deps: Deps, address: String) -> StdResult<Uint128> {
    NUMBERS.load(deps.storage, &address)
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn test_set_number() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("sender", &[]);

        let msg = InstantiateMsg {};

        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SetNumber {
            number: Uint128::new(12),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(
            Response::new()
                .add_attribute("action", "set_number")
                .add_attribute("number", "12"),
            res
        );

        assert_eq!(
            Uint128::new(12),
            NUMBERS.load(deps.as_ref().storage, "sender").unwrap()
        );
    }
}
