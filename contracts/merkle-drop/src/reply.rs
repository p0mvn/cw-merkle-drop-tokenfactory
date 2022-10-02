use cosmwasm_std::{Response, DepsMut, Reply, StdError, SubMsgResult, SubMsgResponse};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMintResponse;

use crate::{ContractError, state::CLAIM};


pub fn handle_mint_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), events }) = msg.result {
        // make sure we parse the desired response correctly.
        let _res: MsgMintResponse = b.try_into().map_err(ContractError::Std)?;

        for event in events {
            deps.api.debug(&event.ty);
        }

        return Ok(Response::default())
    }
    
    

    Err(ContractError::FailedToMint { })
}
