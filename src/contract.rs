use std::ops::Add;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Uint128, Uint64, StdError, OverflowError};
use cw2::set_contract_version;
use cw_utils::Scheduled;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Claim, Config, CONFIG, LOCK_BOX_SEQ, Lockbox, LOCKBOXES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw1-lockbox";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = Config {};
    CONFIG.save(deps.storage, &state)?;

    LOCK_BOX_SEQ.save(deps.storage, &Uint64::zero())?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateLockbox {
            owner,
            claims,
            expiration,
        } => execute_create_lockbox(deps, env, info, owner, claims, expiration),
        ExecuteMsg::Reset {} => unimplemented!(),
    }
}

pub fn execute_create_lockbox(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    owner: String,
    claims: Vec<Claim>,
    expiration: Scheduled,
) -> Result<Response, ContractError> {
    let owner = deps.api.addr_validate(&owner)?;

    if expiration.is_triggered(&env.block) {
        return Err(ContractError::LockboxExpired {});
    }

    let total_amount: Uint128 = claims.clone().into_iter().map(|c| c.amount).sum();

    let id = LOCK_BOX_SEQ.update::<_, StdError>(deps.storage, |id| {
        Ok(id.add(Uint64::new(1)))
    })?;

    let lockbox = Lockbox{
        id,
        owner,
        claims,
        expiration,
        total_amount
    };

    LOCKBOXES.save(deps.storage, id.u64(), &lockbox)?;

    Ok(Response::new().add_attribute("method", "execute_create_lockbox"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    /*
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }

     */

    #[test]
    fn create_lockbox() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { admin: "ADMIN".to_string()};
        let info = mock_info("creator", &[]);
        let mut env = mock_env();
        env.block.height = 1;
        let _res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

        let claims = vec![
            Claim{ addr: "claim1".to_string(), amount: Uint128::new(4) },
            Claim{ addr: "claim2".to_string(), amount: Uint128::new(15) },
        ];
        let msg = ExecuteMsg::CreateLockbox{
            owner: "OWNER".to_string(),
            claims,
            expiration: Scheduled::AtHeight(5)
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(err, ContractError::LockboxExpired{});

    }
}
