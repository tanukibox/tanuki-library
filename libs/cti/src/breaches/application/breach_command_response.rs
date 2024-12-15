use cqrs::domain::command_bus_response::CommandBusResponse;

use crate::shared::domain::errors::DomainError;

pub struct BreachCommandResponse {
    pub error: Option<DomainError>,
}

impl BreachCommandResponse {
    pub const RES_TYPE: &'static str = "BreachCommandResponse";

    pub fn ok() -> BreachCommandResponse {
        BreachCommandResponse { error: None }
    }

    pub fn boxed_ok() -> Box<BreachCommandResponse> {
        let res = BreachCommandResponse::ok();
        Box::new(res)
    }

    pub fn err(error: DomainError) -> BreachCommandResponse {
        BreachCommandResponse { error: Some(error) }
    }

    pub fn boxed_err(error: DomainError) -> Box<BreachCommandResponse> {
        let res = BreachCommandResponse::err(error);
        Box::new(res)
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }

    pub fn error(&self) -> &DomainError {
        self.error.as_ref().unwrap()
    }
}

impl CommandBusResponse for BreachCommandResponse {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
