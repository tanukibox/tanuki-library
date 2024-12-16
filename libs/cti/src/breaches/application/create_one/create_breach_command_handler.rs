use async_trait::async_trait;
use cqrs::domain::{
    command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler,
    query_bus::QueryBus,
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
    cves::domain::entities::cve_id::CveId,
};

use super::{breach_creator::BreachCreator, create_breach_command::CreateBreachCommand};

pub struct CreateBreachCommandHandler<R: BreachRepository, Q: QueryBus, E: EventBus> {
    creator: BreachCreator<R, Q, E>,
}

impl<R: BreachRepository, Q: QueryBus, E: EventBus> CreateBreachCommandHandler<R, Q, E> {
    pub fn new(creator: BreachCreator<R, Q, E>) -> CreateBreachCommandHandler<R, Q, E> {
        CreateBreachCommandHandler { creator }
    }
}

#[async_trait]
impl<R: BreachRepository, Q: QueryBus, E: EventBus> CommandHandler
    for CreateBreachCommandHandler<R, Q, E>
{
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

        match self
            .creator
            .run(id, vendor, product, product_version, product_type, cve_id)
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
