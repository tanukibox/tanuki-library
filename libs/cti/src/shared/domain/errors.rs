use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    //                    GENERAL ERRORS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    #[error("Unknown error.")]
    Unknown,

    #[error("Not valid format for value <{value:?}>.")]
    ValueObjectError { value: String },
    
}