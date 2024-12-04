use std::sync::Arc;

use events::domain::event_bus::EventBus;

use crate::{cves::domain::{entities::{cve::Cve, cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState}, events::cve_created_event::CveCreatedEvent, repositories::cve_repository::CveRepository}, shared::domain::errors::DomainError};


pub struct CveCreator<R: CveRepository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: CveRepository, E: EventBus> CveCreator<R, E> {
    pub fn new(cve_repository: Arc<R>, event_bus: Arc<E>) -> CveCreator<R, E> {
        CveCreator { repository: cve_repository, event_bus }
    }

    pub async fn run(
        &self,
        id: CveId,
        state: CveState,
        date_published: CvePublicationDate,
        description: CveDescription,
    ) -> Result<(), DomainError> {
        tracing::debug!("Starting CVE creation for {}.", id);
        let cve = Cve::from(&id, &state, &date_published, &description);
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
