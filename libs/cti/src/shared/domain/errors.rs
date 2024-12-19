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


    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    //                   BREACH ERRORS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    
    #[error("Breach with <{cve_id:?}, {vendor:?}, {product:?}, {product_version:?}> already exists.")]
    BreachAlreadyExists { cve_id: String, vendor: String, product: String, product_version: String },

    #[error("Breach with id <{cve_id:?}, {vendor:?}, {product:?}, {product_version:?}> not found.")]
    BreachNotFound { cve_id: String, vendor: String, product: String, product_version: String },

    #[error("Breach with id <{cve_id:?}, {vendor:?}, {product:?}, {product_version:?}> not authorized.")]
    BreachNotAuthorized { cve_id: String, vendor: String, product: String, product_version: String },
}
