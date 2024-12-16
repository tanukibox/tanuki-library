use std::sync::Arc;

use cqrs::domain::query_bus::QueryBus;
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
    cves::{application::{cve_query_response::CveQueryResponse, find_one::find_cve_query::FindCveQuery}, domain::entities::{
        cve::Cve, cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName, cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState, cve_updated_date::CveUpdatedDate
    }},
    shared::domain::errors::DomainError,
};

pub struct BreachCreator<R: BreachRepository, Q: QueryBus, E: EventBus> {
    repository: Arc<R>,
    query_bus: Arc<Q>,
    event_bus: Arc<E>,
}

impl<R: BreachRepository, Q: QueryBus, E: EventBus> BreachCreator<R, Q, E> {
    pub fn new(breach_repository: Arc<R>, query_bus: Arc<Q>, event_bus: Arc<E>) -> BreachCreator<R, Q, E> {
        BreachCreator {
            repository: breach_repository,
            query_bus,
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
        tracing::debug!("Finding CVE with id: {}.", cve_id);
        let cve = match self.find_cve(&cve_id).await {
            Ok(cve) => cve,
            Err(err) => return Err(err),
        };
        tracing::debug!("CVE with id: {} found.", cve_id);

        let breach = Breach::from(
            &id,
            &vendor,
            &product,
            &product_version,
            &product_type,
            &cve.id,
            &cve.state,
            &cve.description,
            &cve.assigner_id,
            &cve.assigner_name,
            &cve.date_published,
            &cve.date_updated,
        );
        let res = self.repository.create_one(&breach).await;
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

    async fn find_cve(&self, cve_id: &CveId) -> Result<Cve, DomainError> {
        let query = FindCveQuery::new(Some(cve_id.value()));
        let query = Box::new(query);
        let res = self.query_bus.ask(query).await;
        let res = match res.as_any().downcast_ref::<CveQueryResponse>() {
            Some(res) => res,
            None => panic!("Error getting CVE."),
        };
        
        if res.is_ok() {
            let cve = res.cve.as_ref().unwrap().clone();
            return Ok(cve);
        }

        match &res.error {
            None => panic!("Error getting CVE."),
            Some(err) => return Err(err.clone()),
        }
    }
}
