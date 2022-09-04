#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetMerkleRootResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

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
    let state = State {
        merkle_root: msg.merkle_root,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::LazyMint {
            message_hash,
            signature,
            public_key,
        } => try_lazy_mint(deps, message_hash, signature, public_key),
    }
}

pub fn try_lazy_mint(
    deps: DepsMut,
    message_hash: Vec<u8>,
    signature: Vec<u8>,
    public_key: Vec<u8>,
) -> Result<Response, ContractError> {
    let verify_result = deps
        .api
        .secp256k1_verify(&message_hash, &signature, &public_key);

    let is_verified = match verify_result {
        Ok(result) => result,
        Err(error) => false,
    };

    if !is_verified {
        return Err(ContractError::Unauthorized {});
    }

    // STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    //     state.count += 1;
    //     Ok(state)
    // })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_root(deps)?),
    }
}

fn query_root(deps: Deps) -> StdResult<GetMerkleRootResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetMerkleRootResponse { root: state.merkle_root })
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

        let msg = InstantiateMsg { merkle_root: String::from(TEST_ROOT) };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetMerkleRootResponse = from_binary(&res).unwrap();
        assert_eq!(TEST_ROOT, value.root);
    }
}
