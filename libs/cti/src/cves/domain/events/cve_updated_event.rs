use events::domain::domain_event::DomainEvent;

use crate::cves::domain::entities::{
    cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName, cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState, cve_updated_date::CveUpdatedDate
};

pub struct CveUpdatedEvent {
    pub id: String,

    pub cve_id: CveId,
    pub cve_state: CveState,
    pub cve_old_state: CveState,
    pub cve_description: CveDescription,
    pub cve_old_description: CveDescription,
    pub cve_assigner_id: CveAssignerId,
    pub cve_old_assigner_id: CveAssignerId,
    pub cve_assigner_name: CveAssignerName,
    pub cve_old_assigner_name: CveAssignerName,
    pub cve_date_published: CvePublicationDate,
    pub cve_old_date_published: CvePublicationDate,
    pub cve_updated_date: CveUpdatedDate,
    pub cve_old_updated_date: CveUpdatedDate,

    pub occurred_on: String,
}

impl CveUpdatedEvent {
    pub fn new(
        cve_id: &CveId,
        cve_state: &CveState,
        cve_old_state: &CveState,
        cve_description: &CveDescription,
        cve_old_description: &CveDescription,
        cve_assigner_id: &CveAssignerId,
        cve_old_assigner_id: &CveAssignerId,
        cve_assigner_name: &CveAssignerName,
        cve_old_assigner_name: &CveAssignerName,
        cve_date_published: &CvePublicationDate,
        cve_old_date_published: &CvePublicationDate,
        cve_updated_date: &CveUpdatedDate,
        cve_old_updated_date: &CveUpdatedDate,
        
    ) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let occurred_on = chrono::Utc::now().to_rfc3339();
        Self {
            id,
            cve_id: cve_id.clone(),
            cve_state: cve_state.clone(),
            cve_old_state: cve_old_state.clone(),
            cve_description: cve_description.clone(),
            cve_old_description: cve_old_description.clone(),
            cve_assigner_id: cve_assigner_id.clone(),
            cve_old_assigner_id: cve_old_assigner_id.clone(),
            cve_assigner_name: cve_assigner_name.clone(),
            cve_old_assigner_name: cve_old_assigner_name.clone(),
            cve_date_published: cve_date_published.clone(),
            cve_old_date_published: cve_old_date_published.clone(),
            cve_updated_date: cve_updated_date.clone(),
            cve_old_updated_date: cve_old_updated_date.clone(),
            occurred_on,
        }
    }

    pub fn new_shared(
        cve_id: &CveId,
        cve_state: &CveState,
        cve_old_state: &CveState,
        cve_description: &CveDescription,
        cve_old_description: &CveDescription,
        cve_assigner_id: &CveAssignerId,
        cve_old_assigner_id: &CveAssignerId,
        cve_assigner_name: &CveAssignerName,
        cve_old_assigner_name: &CveAssignerName,
        cve_date_published: &CvePublicationDate,
        cve_old_date_published: &CvePublicationDate,
        cve_updated_date: &CveUpdatedDate,
        cve_old_updated_date: &CveUpdatedDate,
    ) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self::new(
            cve_id,
            cve_state,
            cve_old_state,
            cve_description,
            cve_old_description,
            cve_assigner_id,
            cve_old_assigner_id,
            cve_assigner_name,
            cve_old_assigner_name,
            cve_date_published,
            cve_old_date_published,
            cve_updated_date,
            cve_old_updated_date,
        ))
    }
}

impl DomainEvent for CveUpdatedEvent {
    fn event_type(&self) -> String {
        "com.tanukibox.cti.cve.updated@1.0.0".to_string()
    }
}

impl Clone for CveUpdatedEvent {
    fn clone(&self) -> Self {
        let mut event = Self::new(
            &self.cve_id,
            &self.cve_state,
            &self.cve_old_state,
            &self.cve_description,
            &self.cve_old_description,
            &self.cve_assigner_id,
            &self.cve_old_assigner_id,
            &self.cve_assigner_name,
            &self.cve_old_assigner_name,
            &self.cve_date_published,
            &self.cve_old_date_published,
            &self.cve_updated_date,
            &self.cve_old_updated_date,
        );
        event.occurred_on = self.occurred_on.clone();
        return event;
    }
}
