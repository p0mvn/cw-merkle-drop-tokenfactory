use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unauthorized sender: {sender:?}, owner: {owner:?} ")]
    UnauthorizedSender { sender: String, owner: String },

    #[error("Failed to decode root: {root:?}")]
    FailedToDecodeRoot { root: String },

    #[error("Failed to verify proof")]
    FailedVerifyProof {},

    #[error("{claim:?} already claimed")]
    AlreadyClaimed { claim: String },

    #[error("{reply_id:?} unknown reply id")]
    UnknownReplyId { reply_id: u64 },

    #[error("Failed to mint")]
    FailedToMint {},

    #[error("AuthZ grant for tokenfactory mint is not issued for the contract address")]
    NoAuthZMintGrant {},
}
