use cosmwasm_std::{DepsMut, Reply, Response, SubMsgResponse, SubMsgResult};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMintResponse;

use crate::ContractError;

pub fn handle_mint_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    deps.api.debug(&"mint reply reached");

    match msg.result {
        SubMsgResult::Ok(SubMsgResponse { events, .. }) => {
            deps.api.debug(&"mint reply parsing response");

            deps.api.debug(&format!("{}", events.len()));

            for event in events {
                deps.api.debug(&format!("event = {event:?}"));
            }

            return Ok(Response::new().add_attribute("reply", "tf_mint"));
        }
        SubMsgResult::Err(e) => {
            deps.api.debug(&e);
        }
    }

    Err(ContractError::FailedToMint {})
}
