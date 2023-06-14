use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Subscription cannot be canceled")]
    NotCancelable {},

    #[error("Subscription is already canceled")]
    AlreadyCanceled {},

    #[error("Subscription is already expired")]
    AlreadyExpired {},
}
