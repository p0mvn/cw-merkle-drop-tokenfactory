use cosmwasm_std::{DepsMut, Reply, Response, SubMsg, SubMsgResponse, SubMsgResult};
use osmosis_std::shim::Any;
use osmosis_std::types::cosmos::authz::v1beta1::MsgExec;
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_std::types::cosmos::base::v1beta1::Coin;

use crate::state::CONFIG;
use crate::{
    execute::set_subdenom::BANK_SEND_TYPE_URL,
    state::{CLAIMED_ADDRESSES, REPLY_STATE},
    ContractError,
};

pub const AUTHZ_EXEC_MINT_MSG_ID: u64 = 1;
pub const AUTHZ_EXEC_SEND_MSG_ID: u64 = 2;

pub fn handle_mint_reply(
    deps: DepsMut,
    msg: Reply,
    contract_address: String,
) -> Result<Response, ContractError> {
    deps.api.debug("mint reply reached");

    match msg.result {
        SubMsgResult::Ok(SubMsgResponse { .. }) => {
            deps.api.debug("mint reply parsing response");

            let mint_reply_state = REPLY_STATE.load(deps.storage, AUTHZ_EXEC_MINT_MSG_ID)?;

            let owner = CONFIG.load(deps.storage)?.owner;

            let msg_send = MsgSend {
                from_address: owner.to_string(),
                to_address: mint_reply_state.claimer_addr,
                amount: vec![Coin {
                    denom: mint_reply_state.denom,
                    amount: mint_reply_state.amount.to_string(),
                }],
            };

            let msg_send_binary: cosmwasm_std::Binary = msg_send.into();

            let msg_send_any = Any {
                type_url: String::from(BANK_SEND_TYPE_URL),
                value: msg_send_binary.to_vec(),
            };

            let exec_msg = MsgExec {
                grantee: contract_address,
                msgs: vec![msg_send_any],
            };

            return Ok(Response::new()
                .add_submessage(SubMsg::reply_on_success(exec_msg, AUTHZ_EXEC_SEND_MSG_ID))
                .add_attribute("reply", "tf_mint"));
        }
        SubMsgResult::Err(e) => {
            deps.api.debug(&e);
        }
    }

    Err(ContractError::FailedToMint {})
}

pub fn handle_send_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    deps.api.debug("send reply reached");

    match msg.result {
        SubMsgResult::Ok(SubMsgResponse { .. }) => {
            deps.api.debug("send reply parsing response");

            let mint_reply_state = REPLY_STATE.load(deps.storage, AUTHZ_EXEC_MINT_MSG_ID)?;

            CLAIMED_ADDRESSES.save(deps.storage, &mint_reply_state.claimer_addr, &true)?;

            // Prune mint reply state
            REPLY_STATE.remove(deps.storage, AUTHZ_EXEC_MINT_MSG_ID);

            return Ok(Response::new()
                .add_attribute("reply", "send")
                .add_attribute("merkle-drop-denom", mint_reply_state.denom)
                .add_attribute("merkle-drop-amount", mint_reply_state.amount.to_string())
                .add_attribute("merkle-drop-receiver", mint_reply_state.claimer_addr));
        }
        SubMsgResult::Err(e) => {
            deps.api.debug(&e);
        }
    }

    Err(ContractError::FailedToMint {})
}
