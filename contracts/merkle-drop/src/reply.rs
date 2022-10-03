use cosmwasm_std::{coins, BankMsg, DepsMut, Reply, Response, SubMsgResponse, SubMsgResult};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMintResponse;

use crate::{
    contract::MINT_MSG_ID,
    state::{CLAIMED_ADDRESSES, MINT_REPLY_STATE},
    ContractError,
};

pub fn handle_mint_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    deps.api.debug(&"mint reply reached");

    match msg.result {
        SubMsgResult::Ok(SubMsgResponse { data, events }) => {
            deps.api.debug(&"mint reply parsing response");

            let mint_reply_state = MINT_REPLY_STATE.load(deps.storage, MINT_MSG_ID)?;

            CLAIMED_ADDRESSES.save(deps.storage, &mint_reply_state.claimer_addr, &true)?;
            
            // Prune mint reply state
            MINT_REPLY_STATE.remove(deps.storage, MINT_MSG_ID);

            // Send the swapped token from contract to the original
            // user who initiated the swap.
            let bank_msg = BankMsg::Send {
                to_address: mint_reply_state.claimer_addr,
                amount: coins(mint_reply_state.amount.u128(), mint_reply_state.denom),
            };

            return Ok(Response::new()
                .add_message(bank_msg)
                .add_attribute("reply", "tf_mint"));
        }
        SubMsgResult::Err(e) => {
            deps.api.debug(&e);
        }
    }

    Err(ContractError::FailedToMint {})
}
