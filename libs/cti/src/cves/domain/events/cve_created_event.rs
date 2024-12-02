
use events::domain::domain_event::DomainEvent;

use crate::cves::domain::entities::{cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState};


pub struct CveCreatedEvent {
    pub id: String,
    
    pub cve_id: CveId,
    pub cve_state: CveState,
    pub cve_date_published: CvePublicationDate,
    pub cve_description: CveDescription,

    pub occurred_on: String,
}

impl CveCreatedEvent {
    pub fn new(
        cve_id: CveId, 
        cve_state: CveState, 
        cve_date_published: CvePublicationDate, 
        cve_description: CveDescription,
    ) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let occurred_on = chrono::Utc::now().to_rfc3339();
        Self { id, cve_id, cve_state, cve_date_published, cve_description, occurred_on }
    }

    pub fn new_shared(
        cve_id: CveId, 
        cve_state: CveState, 
        cve_date_published: CvePublicationDate, 
        cve_description: CveDescription,
    ) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self::new(cve_id, cve_state, cve_date_published, cve_description))
    }
}

impl DomainEvent for CveCreatedEvent {
    fn event_type(&self) -> String {
        "com.tanukibox.cti.cve.created@1.0.0".to_string()
    }
}

impl Clone for CveCreatedEvent {
    fn clone(&self) -> Self {
        let mut event = Self::new(
            self.cve_id.clone(),
            self.cve_state.clone(),
            self.cve_date_published.clone(),
            self.cve_description.clone(),
        );
        event.occurred_on = self.occurred_on.clone();
        return event;
    }
}