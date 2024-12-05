use std::sync::Arc;

use events::domain::event_bus::EventBus;

use crate::{
    cves::domain::{
        entities::cve_id::CveId, events::cve_created_event::CveCreatedEvent,
        repositories::cve_repository::CveRepository,
    },
    shared::domain::errors::DomainError,
};

pub struct CveDeleter<R: CveRepository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: CveRepository, E: EventBus> CveDeleter<R, E> {
    pub fn new(cve_repository: Arc<R>, event_bus: Arc<E>) -> CveDeleter<R, E> {
        CveDeleter {
            repository: cve_repository,
            event_bus,
        }
    }

    pub async fn run(&self, id: CveId) -> Result<(), DomainError> {
        tracing::debug!("Starting CVE deletion for {}.", id);
        let old_cve = self
            .repository
            .find_by_id(&id)
            .await
            .or_else(|e| Err(e))
            .unwrap();
        let res = self.repository.delete_one(&id).await;
        if res.is_err() {
            tracing::info!("Error deleting CVE with id: {}.", id);
            return Err(res.err().unwrap());
        }
        let created_event = CveCreatedEvent::new_shared(
            &id,
            &old_cve.state,
            &old_cve.date_published,
            &old_cve.description,
        );
        self.event_bus.publish(created_event).await;
        tracing::debug!("CVE with id: {} deleted.", id);
        Ok(())
    }
}
