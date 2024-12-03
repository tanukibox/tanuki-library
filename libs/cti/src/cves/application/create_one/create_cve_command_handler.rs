use std::sync::Arc;

use async_trait::async_trait;
use cqrs::domain::{command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler};
use events::domain::event_bus::EventBus;

use crate::cves::{application::cve_command_response::CveCommandResponse, domain::{entities::cve_id::CveId, repositories::cve_repository::CveRepository}};

use super::{create_cve_command::CreateCveCommand, cve_creator::CveCreator};


pub struct CreateCveCommandHandler<R: CveRepository, E: EventBus> {
    creator: Arc<CveCreator<R, E>>,
}

impl<R: CveRepository, E: EventBus>  CreateCveCommandHandler<R, E> {
    pub fn new(creator: Arc<CveCreator<R, E>>) -> CreateCveCommandHandler<R, E> {
        CreateCveCommandHandler { creator }
    }
}

#[async_trait]
impl <R: CveRepository, E: EventBus> CommandHandler for CreateCveCommandHandler<R, E> {
    async fn handle(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse> {
        let command = command.as_any().downcast_ref::<CreateCveCommand>().unwrap();

        let id = match CveId::from_optional(&command.id) {
            Ok(id) => id,
            Err(err) => return CveCommandResponse::boxed_err(err)  
        };


        // let res = self.creator.run(id).await;

        /*match res {
            Ok(_) => CveCommandResponse::boxed_ok(),
            Err(err) => CryptoKeyCommandResponse::boxed_err(err)
        }*/
        unimplemented!()
    }
    
    fn subscribet_to(&self) -> String {
        return CreateCveCommand::COMMAND_TYPE.to_string();
    }
}


