use cosmwasm_std::{DepsMut, Reply, Response, SubMsgResponse, SubMsgResult};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMintResponse;

use crate::ContractError;

pub fn handle_mint_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    deps.api.debug(&"mint reply reached");

    if let SubMsgResult::Ok(SubMsgResponse {
        data: Some(b),
        events,
    }) = msg.result
    {
        deps.api.debug(&"mint reply parsing response");
        // make sure we parse the desired response correctly.
        let _res: MsgMintResponse = b.try_into().map_err(ContractError::Std)?;

        for event in events {
            deps.api.debug(&event.ty);
        }

        return Ok(Response::default());
    }

    Err(ContractError::FailedToMint {})
}
