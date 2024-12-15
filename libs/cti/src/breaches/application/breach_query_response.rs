use cqrs::domain::query_bus_response::QueryBusResponse;

use crate::{breaches::domain::entities::breach::Breach, shared::domain::errors::DomainError};

pub struct BreachQueryResponse {
    pub error: Option<DomainError>,
    pub breach: Option<Breach>,
}

impl BreachQueryResponse {
    pub const RES_TYPE: &'static str = "BreachQueryResponse";

    pub fn ok(breach: Breach) -> BreachQueryResponse {
        BreachQueryResponse {
            error: None,
            breach: Some(breach),
        }
    }

    pub fn boxed_ok(breach: Breach) -> Box<BreachQueryResponse> {
        let res = BreachQueryResponse::ok(breach);
        Box::new(res)
    }

    pub fn err(error: DomainError) -> BreachQueryResponse {
        BreachQueryResponse {
            error: Some(error),
            breach: None,
        }
    }

    pub fn boxed_err(error: DomainError) -> Box<BreachQueryResponse> {
        let res = BreachQueryResponse::err(error);
        Box::new(res)
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }
}

impl QueryBusResponse for BreachQueryResponse {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
