use cosmwasm_std::{DepsMut, Env, MessageInfo, QuerierWrapper, Response, StdError};
use osmosis_std::types::{
    cosmos::authz::v1beta1::AuthzQuerier,
    osmosis::tokenfactory::v1beta1::{MsgMint, TokenfactoryQuerier},
};

use crate::{
    state::{CONFIG, SUBDENOM},
    ContractError,
};

pub const BANK_SEND_TYPE_URL: &str = "/cosmos.bank.v1beta1.MsgSend";

pub fn set_subdenom(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    subdenom: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // validate sender
    if config.owner != info.sender {
        return Err(ContractError::UnauthorizedSender {
            sender: info.sender.into_string(),
            owner: config.owner.into_string(),
        });
    }

    // validate that subdenom exists and that owner is admin
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
    if !admin.eq(&config.owner.to_string()) {
        return Err(ContractError::Unauthorized {});
    }

    // ensure that authz grants are created for tokenfactory mint and bank send.
    validate_grant(
        &deps.querier,
        config.owner.as_str(),
        env.contract.address.as_str(),
        MsgMint::TYPE_URL,
    )?;
    validate_grant(
        &deps.querier,
        config.owner.as_str(),
        env.contract.address.as_str(),
        BANK_SEND_TYPE_URL,
    )?;

    SUBDENOM.save(deps.storage, &subdenom)?;

    deps.api.debug(&format!("saved subdenom {0}", &subdenom));

    Ok(Response::new()
        .add_attribute("method", "set_subdenom")
        .add_attribute("owner", info.sender)
        .add_attribute("subdenom", subdenom))
}

fn validate_grant(
    querier: &QuerierWrapper,
    granter: &str,
    grantee: &str,
    msg_type_url: &str,
) -> Result<(), ContractError> {
    // ensure that authz grant is created
    let authz_querier = AuthzQuerier::new(querier);

    let grants_response = authz_querier.grants(
        String::from(granter),
        String::from(grantee),
        String::from(msg_type_url),
        Option::None,
    )?;

    if grants_response.grants.is_empty() {
        // TODO: format addresses
        return Err(ContractError::NoAuthZMintGrant {});
    }

    Ok(())
}
