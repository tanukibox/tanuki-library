use std::sync::Arc;

use events::domain::event_bus::EventBus;
use tracing::debug;

use crate::{cves::domain::{entities::{cve::Cve, cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState}, events::cve_created_event::CveCreatedEvent, repositories::cve_repository::CveRepository}, shared::domain::errors::DomainError};



pub struct CryptoKeyCreator<R: CveRepository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: CveRepository, E: EventBus> CryptoKeyCreator<R, E> {
    pub fn new(cve_repository: Arc<R>, event_bus: Arc<E>) -> CryptoKeyCreator<R, E> {
        CryptoKeyCreator { repository: cve_repository, event_bus }
    }

    pub async fn run(
        &self,
        id: CveId,
        state: CveState,
        date_published: CvePublicationDate,
        description: CveDescription,
    ) -> Result<(), DomainError> {
        debug!("Starting CVE creation");
        let key = Cve::new(id.clone(), state.clone(), date_published.clone(), description.clone());
        let res = self.repository.create_one(&key).await;
        if res.is_err() {
            debug!("Error creating CVE with id: {}", id.value());
            return Err(res.err().unwrap());
        }
        let created_event = CveCreatedEvent::new_shared(id.clone(), state, date_published, description);
        self.event_bus.publish(created_event).await;
        debug!("CVE with id: {} created", id.value());
        Ok(())
    }
}
