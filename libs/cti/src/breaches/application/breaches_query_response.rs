use cqrs::domain::query_bus_response::QueryBusResponse;

use crate::{breaches::domain::entities::breach::Breach, shared::domain::errors::DomainError};

pub struct BreachesQueryResponse {
    pub error: Option<DomainError>,
    pub breaches: Vec<Breach>,
}

impl BreachesQueryResponse {
    pub const RES_TYPE: &'static str = "BreachesQueryResponse";

    pub fn ok(breaches: Vec<Breach>) -> BreachesQueryResponse {
        BreachesQueryResponse {
            error: None,
            breaches,
        }
    }

    pub fn boxed_ok(breaches: Vec<Breach>) -> Box<BreachesQueryResponse> {
        let res = BreachesQueryResponse::ok(breaches);
        Box::new(res)
    }

    pub fn err(error: DomainError) -> BreachesQueryResponse {
        BreachesQueryResponse {
            error: Some(error),
            breaches: vec![],
        }
    }

    pub fn boxed_err(error: DomainError) -> Box<BreachesQueryResponse> {
        let res = BreachesQueryResponse::err(error);
        Box::new(res)
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }
}

impl QueryBusResponse for BreachesQueryResponse {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
