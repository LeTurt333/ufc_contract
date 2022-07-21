use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Serialization Error")]
    CerealError {},

    #[error("Undefined Error...")]
    UndefinedError {},

    #[error("Unimplemented execute message")]
    UnimplementedMessage {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Error adding fight")]
    AddFightError {},
}
