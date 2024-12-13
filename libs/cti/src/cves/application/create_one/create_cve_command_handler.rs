use async_trait::async_trait;
use cqrs::domain::{
    command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler,
};
use events::domain::event_bus::EventBus;

use crate::cves::{
    application::cve_command_response::CveCommandResponse,
    domain::{
        entities::{
            cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName, cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState, cve_updated_date::CveUpdatedDate
        },
        repositories::cve_repository::CveRepository,
    },
};

use super::{create_cve_command::CreateCveCommand, cve_creator::CveCreator};

pub struct CreateCveCommandHandler<R: CveRepository, E: EventBus> {
    creator: CveCreator<R, E>,
}

impl<R: CveRepository, E: EventBus> CreateCveCommandHandler<R, E> {
    pub fn new(creator: CveCreator<R, E>) -> CreateCveCommandHandler<R, E> {
        CreateCveCommandHandler { creator }
    }
}

#[async_trait]
impl<R: CveRepository, E: EventBus> CommandHandler for CreateCveCommandHandler<R, E> {
    async fn handle(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse> {
        let command = command.as_any().downcast_ref::<CreateCveCommand>().unwrap();

        let id = match CveId::from_optional(&command.id) {
            Ok(id) => id,
            Err(err) => return CveCommandResponse::boxed_err(err),
        };

        let state = match CveState::from_optional(&command.state) {
            Ok(state) => state,
            Err(err) => return CveCommandResponse::boxed_err(err),
        };

        let description = match CveDescription::new(&command.description) {
            Ok(description) => description,
            Err(err) => return CveCommandResponse::boxed_err(err),
        };

        let assigner_id = match CveAssignerId::from_optional(&command.assigner_id) {
            Ok(assigner_id) => assigner_id,
            Err(err) => return CveCommandResponse::boxed_err(err),
        };

        let assigner_name = match CveAssignerName::from_optional(&command.assigner_name) {
            Ok(assigner_name) => assigner_name,
            Err(err) => return CveCommandResponse::boxed_err(err),
        };

        let publication_date = match CvePublicationDate::from_optional(&command.date_published) {
            Ok(publication_date) => publication_date,
            Err(err) => return CveCommandResponse::boxed_err(err),
        };

        let updated_date = match CveUpdatedDate::from_optional(&command.date_updated) {
            Ok(updated_date) => updated_date,
            Err(err) => return CveCommandResponse::boxed_err(err),
        };

        match self
            .creator
            .run(id, state, description, assigner_id, assigner_name, publication_date, updated_date)
            .await
        {
            Ok(_) => CveCommandResponse::boxed_ok(),
            Err(err) => CveCommandResponse::boxed_err(err),
        }
    }

    fn subscribet_to(&self) -> String {
        return CreateCveCommand::COMMAND_TYPE.to_string();
    }
}
