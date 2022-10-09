use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::claim::claim;
use crate::execute::set_subdenom::set_subdenom;
use crate::msg::{ExecuteMsg, GetRootResponse, GetSubdenomResponse, InstantiateMsg, QueryMsg};
use crate::reply::{
    handle_mint_reply, handle_send_reply, AUTHZ_EXEC_MINT_MSG_ID, AUTHZ_EXEC_SEND_MSG_ID,
};
use crate::state::{Config, CONFIG, SUBDENOM};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:merkle-drop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        merkle_root: msg.merkle_root,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

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
        ExecuteMsg::SetSubDenom { subdenom } => set_subdenom(deps, env, info, subdenom),
        ExecuteMsg::Claim {
            proof,
            amount,
            claimer_addr,
        } => claim(deps, env, proof, amount, claimer_addr),
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    deps.api.debug("reply reached");
    if msg.id == AUTHZ_EXEC_MINT_MSG_ID {
        return handle_mint_reply(deps, msg, env.contract.address.to_string());
    } else if msg.id == AUTHZ_EXEC_SEND_MSG_ID {
        return handle_send_reply(deps, msg);
    }
    Err(ContractError::UnknownReplyId { reply_id: msg.id })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRoot {} => to_binary(&query_root(deps)?),
        QueryMsg::GetSubdenom {} => to_binary(&query_subdenom(deps)?),
    }
}

fn query_root(deps: Deps) -> StdResult<GetRootResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(GetRootResponse {
        root: config.merkle_root,
    })
}

fn query_subdenom(deps: Deps) -> StdResult<GetSubdenomResponse> {
    let subdenom = SUBDENOM.load(deps.storage)?;

    deps.api
        .debug(&format!("returning subdenom {0}", &subdenom));

    Ok(GetSubdenomResponse { subdenom })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    // TEST_ROOT test merkel root that was generated from "testdata/uosmo_only.csv" using merkle-drop-cli
    const TEST_ROOT: &str = "bd9c439f3903b3dbc92bad230df593d434aada80f26e8124d77d2f92fbaa6238";

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            merkle_root: String::from(TEST_ROOT),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetRoot {}).unwrap();
        let value: GetRootResponse = from_binary(&res).unwrap();
        assert_eq!(TEST_ROOT, value.root);
    }
}
