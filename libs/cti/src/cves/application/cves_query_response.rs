use cqrs::domain::query_bus_response::QueryBusResponse;

use crate::{cves::domain::entities::cve::Cve, shared::domain::errors::DomainError};


pub struct CvesQueryResponse {
    pub error: Option<DomainError>,
    pub cves: Vec<Cve>,
}

impl CvesQueryResponse {
    pub const RES_TYPE: &'static str = "CvesQueryResponse";

    pub fn ok(cves: Vec<Cve>) -> CvesQueryResponse {
        CvesQueryResponse { error: None, cves }
    }

    pub fn boxed_ok(cves: Vec<Cve>) -> Box<CvesQueryResponse> {
        let res = CvesQueryResponse::ok(cves);
        Box::new(res)
    }

    pub fn err(error: DomainError) -> CvesQueryResponse {
        CvesQueryResponse { error: Some(error), cves: vec![] }
    }

    pub fn boxed_err(error: DomainError) -> Box<CvesQueryResponse> {
        let res = CvesQueryResponse::err(error);
        Box::new(res)
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }
}

impl QueryBusResponse for CvesQueryResponse {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}