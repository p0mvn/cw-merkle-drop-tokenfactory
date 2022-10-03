#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg, Uint128,
};
use cw2::set_contract_version;
use osmosis_std::types::cosmos::base::v1beta1;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgChangeAdmin, MsgMint, TokenfactoryQuerier,
};

use crate::error::ContractError;
use crate::execute::verify_proof;
use crate::msg::{ExecuteMsg, GetRootResponse, GetSubDenomResponse, InstantiateMsg, QueryMsg};
use crate::reply::handle_mint_reply;
use crate::state::{Config, CLAIM, CONFIG, SUBDENOM};

// type Grant struct {
// 	Authorization *types.Any `protobuf:"bytes,1,opt,name=authorization,proto3" json:"authorization,omitempty"`
// 	Expiration    time.Time  `protobuf:"bytes,2,opt,name=expiration,proto3,stdtime" json:"expiration"`
// }

// pub struct GrantMsg {
//     Granter string `protobuf:"bytes,1,opt,name=granter,proto3" json:"granter,omitempty"`
// 	Grantee string `protobuf:"bytes,2,opt,name=grantee,proto3" json:"grantee,omitempty"`
// 	Grant   Grant  `protobuf:"bytes,3,opt,name=grant,proto3" json:"grant"`
// }

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:merkle-drop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const MINT_MSG_ID: u64 = 1;

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
        } => claim(deps, env, info, proof, amount, claimer_addr),
    }
}

pub fn set_subdenom(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    subdenom: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // validate sender
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // validate that subdenom exists and that contract is admin
    let tf_querier = TokenfactoryQuerier::new(&deps.querier);
    let full_denom = format!("factory/{}/{}", config.owner, subdenom);
    deps.api
        .debug(&format!("set_subdenom full_denom: {}", full_denom));
    let response = tf_querier.denom_authority_metadata(full_denom)?;

    if response.authority_metadata.is_none() {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: String::from("invalid authority metadata"),
        }));
    }

    let admin = response.authority_metadata.unwrap().admin;
    deps.api.debug(&format!("denom admin = {admin:?}"));
    if !admin.eq(&env.contract.address) {
        return Err(ContractError::Unauthorized {});
    }

    SUBDENOM.save(deps.storage, &subdenom)?;

    deps.api.debug(&format!("saved subdenom {0}", &subdenom));

    Ok(Response::new()
        .add_attribute("method", "set_subdenom")
        .add_attribute("owner", info.sender)
        .add_attribute("subdenom", subdenom))
}

pub fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    proof_str: String,
    amount: Uint128,
    claimer_addr: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage).unwrap();

    // TODO: validate claimer_addr is an actual account

    let claim = format!("{}{}", claimer_addr, amount.to_string());

    let claim_check = CLAIM.may_load(deps.storage, &claim)?;
    if claim_check.is_some() {
        return Err(ContractError::AlreadyClaimed {
            claim: claim.clone(),
        });
    }

    deps.api
        .debug(&format!("merkle_root {0}", &config.merkle_root));

    deps.api.debug(&format!("proof_str {0}", &proof_str));

    deps.api.debug(&format!("claim {0}", &claim));

    verify_proof(&config.merkle_root, &proof_str, &claim)?;

    deps.api.debug(&"validation passed");

    let subdenom = SUBDENOM.load(deps.storage)?;

    let full_denom = format!("factory/{}/{}", config.owner, subdenom);
    deps.api
        .debug(&format!("claim full_denom: claim end: {}", full_denom));

    let tf_querier = TokenfactoryQuerier::new(&deps.querier);
    let admin = tf_querier
        .denom_authority_metadata(full_denom.clone())?
        .authority_metadata
        .unwrap()
        .admin;
    deps.api.debug(&format!("denom admin = {admin:?}"));

    let mint_msg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(v1beta1::Coin {
            denom: full_denom,
            amount: amount.to_string(),
        }),
    };

    CLAIM.save(deps.storage, &claim, &true)?;

    deps.api.debug(&"claim end");

    Ok(Response::new()
        .add_attribute("action", "claim")
        .add_submessage(SubMsg::reply_on_success(mint_msg, MINT_MSG_ID)))
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    deps.api.debug(&"reply reached");
    match msg.id {
        MINT_MSG_ID => handle_mint_reply(deps, msg),
        id => Err(ContractError::UnknownReplyId { reply_id: id }),
    }
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

fn query_subdenom(deps: Deps) -> StdResult<GetSubDenomResponse> {
    let subdenom = SUBDENOM.load(deps.storage)?;

    deps.api
        .debug(&format!("returning subdenom {0}", &subdenom));

    Ok(GetSubDenomResponse { subdenom: subdenom })
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
