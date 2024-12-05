use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DomainError {

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    //                    GENERAL ERRORS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    #[error("Unknown error.")]
    Unknown,

    #[error("Not valid format for value <{value:?}>.")]
    ValueObjectError { value: String },

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    //                      CVE ERRORS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    #[error("Cve with id <{id:?}> already exists.")]
    CveAlreadyExists { id: String },

    #[error("Cve with id <{id:?}> not found.")]
    CveNotFound { id: String },

    #[error("Cve with id <{id:?}> not authorized.")]
    CveNotAuthorized { id: String },
    
}