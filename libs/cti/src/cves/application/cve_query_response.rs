use cqrs::domain::query_bus_response::QueryBusResponse;

use crate::{cves::domain::entities::cve::Cve, shared::domain::errors::DomainError};


pub struct CveQueryResponse {
    pub error: Option<DomainError>,
    pub cve: Option<Cve>,
}

impl CveQueryResponse {
    pub const RES_TYPE: &'static str = "CveQueryResponse";

    pub fn ok(cve: Cve) -> CveQueryResponse {
        CveQueryResponse { error: None, cve: Some(cve) }
    }

    pub fn boxed_ok(cve: Cve) -> Box<CveQueryResponse> {
        let res = CveQueryResponse::ok(cve);
        Box::new(res)
    }

    pub fn err(error: DomainError) -> CveQueryResponse {
        CveQueryResponse { error: Some(error), cve: None }
    }

    pub fn boxed_err(error: DomainError) -> Box<CveQueryResponse> {
        let res = CveQueryResponse::err(error);
        Box::new(res)
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }
}

impl QueryBusResponse for CveQueryResponse {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}