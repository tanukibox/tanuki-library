
use cqrs::domain::command_bus_response::CommandBusResponse;

use crate::shared::domain::errors::DomainError;


pub struct CveCommandResponse {
    pub error: Option<DomainError>,
}

impl CveCommandResponse {
    pub const RES_TYPE: &'static str = "CveCommandResponse";

    pub fn ok() -> CveCommandResponse {
        CveCommandResponse { error: None }
    }

    pub fn boxed_ok() -> Box<CveCommandResponse> {
        let res = CveCommandResponse::ok();
        Box::new(res)
    }

    pub fn err(error: DomainError) -> CveCommandResponse {
        CveCommandResponse { error: Some(error) }
    }

    pub fn boxed_err(error: DomainError) -> Box<CveCommandResponse> {
        let res = CveCommandResponse::err(error);
        Box::new(res)
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }
}

impl CommandBusResponse for CveCommandResponse {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}