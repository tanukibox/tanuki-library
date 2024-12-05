use async_trait::async_trait;
use cqrs::domain::{command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler};
use events::domain::event_bus::EventBus;

use crate::cves::{application::cve_command_response::CveCommandResponse, domain::{entities::cve_id::CveId, repositories::cve_repository::CveRepository}};

use super::{cve_deleter::CveDeleter, delete_cve_command::DeleteCveCommand};


pub struct DeleteCveCommandHandler<R: CveRepository, E: EventBus> {
    deleter: CveDeleter<R, E>,
}

impl<R: CveRepository, E: EventBus>  DeleteCveCommandHandler<R, E> {
    pub fn new(deleter: CveDeleter<R, E>) -> DeleteCveCommandHandler<R, E> {
        DeleteCveCommandHandler { deleter }
    }
}

#[async_trait]
impl <R: CveRepository, E: EventBus> CommandHandler for DeleteCveCommandHandler<R, E> {
    
    async fn handle(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse> {
        let command = command.as_any().downcast_ref::<DeleteCveCommand>().unwrap();

        let id = match CveId::from_optional(&command.id) {
            Ok(id) => id,
            Err(err) => return CveCommandResponse::boxed_err(err),
        };

        match self.deleter.run(id).await {
            Ok(_) => CveCommandResponse::boxed_ok(),
            Err(err) => CveCommandResponse::boxed_err(err),
        }
    }
    
    fn subscribet_to(&self) -> String {
        return DeleteCveCommand::COMMAND_TYPE.to_string(); 
    }
}


