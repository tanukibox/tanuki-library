use std::sync::Arc;

use events::domain::event_bus::EventBus;

use crate::{
    cves::domain::{
        entities::{
            cve::Cve, cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName, cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState, cve_updated_date::CveUpdatedDate
        },
        events::cve_updated_event::CveUpdatedEvent,
        repositories::cve_repository::CveRepository,
    },
    shared::domain::errors::DomainError,
};

pub struct CveUpdater<R: CveRepository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: CveRepository, E: EventBus> CveUpdater<R, E> {
    pub fn new(cve_repository: Arc<R>, event_bus: Arc<E>) -> CveUpdater<R, E> {
        CveUpdater {
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
        tracing::debug!("Starting CVE update for {}.", id);
        tracing::debug!("Finding old CVE data {}.", id);
        let old_cve = match self.repository.find_by_id(&id).await {
            Ok(cve) => cve,
            Err(err) => return Err(err),
        };
        tracing::debug!("CVE {} found.", id);
        tracing::debug!("Starting CVE update for {}.", id);
        let cve = Cve::from(&id, &state, &description, &assigner_id, &assigner_name, &date_published, &date_updated);
        if cve == old_cve {
            tracing::info!("CVE with id: {} is already up to date.", id);
            return Ok(());
        }
        let res = self.repository.create_one(&cve).await;
        if res.is_err() {
            tracing::info!("Error updating CVE with id: {}.", id);
            return Err(res.err().unwrap());
        }
        let created_event = CveUpdatedEvent::new_shared(
            &id,
            &state,
            &old_cve.state,
            &description,
            &old_cve.description,
            &assigner_id,
            &old_cve.assigner_id,
            &assigner_name,
            &old_cve.assigner_name,
            &date_published,
            &old_cve.date_published,
            &date_updated,
            &old_cve.date_updated,
        );
        self.event_bus.publish(created_event).await;
        tracing::debug!("CVE with id: {} created.", id);
        Ok(())
    }
}
