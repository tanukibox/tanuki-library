use std::sync::Arc;

use events::domain::event_bus::EventBus;

use crate::{
    cves::domain::{
        entities::{
            cve::Cve, cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName, cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState, cve_updated_date::CveUpdatedDate
        },
        events::cve_created_event::CveCreatedEvent,
        repositories::cve_repository::CveRepository,
    },
    shared::domain::errors::DomainError,
};

pub struct CveCreator<R: CveRepository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: CveRepository, E: EventBus> CveCreator<R, E> {
    pub fn new(cve_repository: Arc<R>, event_bus: Arc<E>) -> CveCreator<R, E> {
        CveCreator {
            repository: cve_repository,
            event_bus,
        }
    }

    pub async fn run(
        &self,
        id: CveId,
        state: CveState,
        description: CveDescription,
        assigner_id: CveAssignerId,
        assigner_name: CveAssignerName,
        date_published: CvePublicationDate,
        date_updated: CveUpdatedDate,
    ) -> Result<(), DomainError> {
        tracing::debug!("Starting CVE creation for {}.", id);
        let cve = Cve::from(&id, &state, &description, &assigner_id, &assigner_name, &date_published, &date_updated);
        let res = self.repository.create_one(&cve).await;
        if res.is_err() {
            tracing::info!("Error creating CVE with id: {}.", id);
            return Err(res.err().unwrap());
        }
        let created_event = CveCreatedEvent::new_shared(&id, &state, &date_published, &description);
        self.event_bus.publish(created_event).await;
        tracing::debug!("CVE with id: {} created.", id);
        Ok(())
    }
}
