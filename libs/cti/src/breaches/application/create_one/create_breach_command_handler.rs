use async_trait::async_trait;
use cqrs::domain::{
    command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler,
};
use events::domain::event_bus::EventBus;

use crate::{
    breaches::{
        application::breach_command_response::BreachCommandResponse,
        domain::{
            entities::{
                breach_id::BreachId, breach_product::BreachProduct,
                breach_product_type::BreachProductType,
                breach_product_version::BreachProductVersion, breach_vendor::BreachVendor,
            },
            repositories::breach_repository::BreachRepository,
        },
    },
    cves::domain::entities::{
        cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName,
        cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate,
        cve_state::CveState, cve_updated_date::CveUpdatedDate,
    },
};

use super::{breach_creator::BreachCreator, create_breach_command::CreateBreachCommand};

pub struct CreateBreachCommandHandler<R: BreachRepository, E: EventBus> {
    creator: BreachCreator<R, E>,
}

impl<R: BreachRepository, E: EventBus> CreateBreachCommandHandler<R, E> {
    pub fn new(creator: BreachCreator<R, E>) -> CreateBreachCommandHandler<R, E> {
        CreateBreachCommandHandler { creator }
    }
}

#[async_trait]
impl<R: BreachRepository, E: EventBus> CommandHandler for CreateBreachCommandHandler<R, E> {
    async fn handle(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse> {
        let command = command
            .as_any()
            .downcast_ref::<CreateBreachCommand>()
            .unwrap();

        let id = match BreachId::from_optional(&command.id) {
            Ok(id) => id,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let vendor = match BreachVendor::from_optional(&command.vendor) {
            Ok(vendor) => vendor,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let product = match BreachProduct::from_optional(&command.product) {
            Ok(product) => product,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let product_version = match BreachProductVersion::from_optional(&command.product_version) {
            Ok(product_version) => product_version,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let product_type = match BreachProductType::from_optional(&command.product_type) {
            Ok(id) => id,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let cve_id = match CveId::from_optional(&command.cve_id) {
            Ok(id) => id,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let cve_state = match CveState::from_optional(&command.cve_state) {
            Ok(state) => state,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let cve_description = match CveDescription::new(&command.cve_description) {
            Ok(description) => description,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let cve_assigner_id = match CveAssignerId::from_optional(&command.cve_assigner_id) {
            Ok(assigner_id) => assigner_id,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let cve_assigner_name = match CveAssignerName::from_optional(&command.cve_assigner_name) {
            Ok(assigner_name) => assigner_name,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        let cve_publication_date =
            match CvePublicationDate::from_optional(&command.cve_date_published) {
                Ok(publication_date) => publication_date,
                Err(err) => return BreachCommandResponse::boxed_err(err),
            };

        let cve_updated_date = match CveUpdatedDate::from_optional(&command.cve_date_updated) {
            Ok(updated_date) => updated_date,
            Err(err) => return BreachCommandResponse::boxed_err(err),
        };

        match self
            .creator
            .run(
                id,
                vendor,
                product,
                product_version,
                product_type,
                cve_id,
                cve_state,
                cve_description,
                cve_assigner_id,
                cve_assigner_name,
                cve_publication_date,
                cve_updated_date,
            )
            .await
        {
            Ok(_) => BreachCommandResponse::boxed_ok(),
            Err(err) => BreachCommandResponse::boxed_err(err),
        }
    }

    fn subscribet_to(&self) -> String {
        return CreateBreachCommand::COMMAND_TYPE.to_string();
    }
}
