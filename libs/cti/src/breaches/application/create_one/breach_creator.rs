use std::sync::Arc;

use events::domain::event_bus::EventBus;

use crate::{
    breaches::domain::{
        entities::{
            breach::Breach, breach_id::BreachId, breach_product::BreachProduct,
            breach_product_type::BreachProductType, breach_product_version::BreachProductVersion,
            breach_vendor::BreachVendor,
        },
        events::breach_created_event::BreachCreatedEvent,
        repositories::breach_repository::BreachRepository,
    },
    cves::domain::entities::{
        cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName,
        cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate,
        cve_state::CveState, cve_updated_date::CveUpdatedDate,
    },
    shared::domain::errors::DomainError,
};

pub struct BreachCreator<R: BreachRepository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: BreachRepository, E: EventBus> BreachCreator<R, E> {
    pub fn new(breach_repository: Arc<R>, event_bus: Arc<E>) -> BreachCreator<R, E> {
        BreachCreator {
            repository: breach_repository,
            event_bus,
        }
    }

    pub async fn run(
        &self,
        id: BreachId,
        vendor: BreachVendor,
        product: BreachProduct,
        product_version: BreachProductVersion,
        product_type: BreachProductType,
        cve_id: CveId,
        cve_state: CveState,
        cve_description: CveDescription,
        cve_assigner_id: CveAssignerId,
        cve_assigner_name: CveAssignerName,
        cve_date_published: CvePublicationDate,
        cve_date_updated: CveUpdatedDate,
    ) -> Result<(), DomainError> {
        tracing::debug!("Starting Breach creation for {}.", id);
        let cve = Breach::from(
            &id,
            &vendor,
            &product,
            &product_version,
            &product_type,
            &cve_id,
            &cve_state,
            &cve_description,
            &cve_assigner_id,
            &cve_assigner_name,
            &cve_date_published,
            &cve_date_updated,
        );
        let res = self.repository.create_one(&cve).await;
        if res.is_err() {
            tracing::info!("Error creating Breach with id: {}.", id);
            return Err(res.err().unwrap());
        }
        let created_event = BreachCreatedEvent::new_shared(
            &id,
            &vendor,
            &product,
            &product_version,
            &product_type,
            &cve_id,
            &cve_state,
            &cve_description,
            &cve_assigner_id,
            &cve_assigner_name,
            &cve_date_published,
            &cve_date_updated,
        );
        self.event_bus.publish(created_event).await;
        tracing::debug!("Breach with id: {} created.", id);
        Ok(())
    }
}
