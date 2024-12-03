
use events::domain::domain_event::DomainEvent;

use crate::cves::domain::entities::{cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState};


pub struct CveCreatedEvent<'a> {
    pub id: String,
    pub cve_id: &'a CveId,
    pub cve_state: &'a CveState,
    pub cve_date_published: &'a CvePublicationDate,
    pub cve_description: &'a CveDescription,
    pub occurred_on: String,
}

impl<'a> CveCreatedEvent<'a> {
    pub fn new(
        cve_id: &'a CveId, 
        cve_state: &'a CveState, 
        cve_date_published: &'a CvePublicationDate, 
        cve_description: &'a CveDescription,
    ) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let occurred_on = chrono::Utc::now().to_rfc3339();
        Self { id, cve_id, cve_state, cve_date_published, cve_description, occurred_on }
    }

    pub fn new_shared(
        cve_id: &'a CveId,
        cve_state: &'a CveState,
        cve_date_published: &'a CvePublicationDate,
        cve_description: &'a CveDescription,
    ) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self::new(cve_id, cve_state, cve_date_published, cve_description))
    }
}

impl<'a> DomainEvent for CveCreatedEvent<'a> {
    fn event_type(&self) -> String {
        "com.tanukibox.cti.cve.created@1.0.0".to_string()
    }
}

impl<'a> Clone for CveCreatedEvent<'a> {
    fn clone(&self) -> Self {
        let mut event = Self::new(
            self.cve_id,
            self.cve_state,
            self.cve_date_published,
            self.cve_description,
        );
        event.occurred_on = self.occurred_on.clone();
        return event;
    }
}