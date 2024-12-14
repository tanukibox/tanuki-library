use sqlx::FromRow;

use crate::cves::domain::entities::{
    cve::Cve, cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName,
    cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate,
    cve_state::CveState, cve_updated_date::CveUpdatedDate,
};

#[derive(Debug, FromRow, Clone)]
pub struct SqlxCve {
    pub id: String,
    pub state: String,
    pub description: Option<String>,
    pub assigner_id: String,
    pub assigner_name: String,
    pub date_published: String,
    pub date_updated: String,
}

impl SqlxCve {
    pub fn to_domain(self) -> Cve {
        Cve::new(
            CveId::new(&self.id).unwrap(),
            CveState::new(&self.state).unwrap(),
            CveDescription::new(&self.description).unwrap(),
            CveAssignerId::new(&self.assigner_id).unwrap(),
            CveAssignerName::new(&self.assigner_name).unwrap(),
            CvePublicationDate::new(&self.date_published).unwrap(),
            CveUpdatedDate::new(&self.date_updated).unwrap(),
        )
    }

    pub fn from_domain(cve: &Cve) -> Self {
        Self {
            id: cve.id.value(),
            state: cve.state.value(),
            description: cve.description.value(),
            assigner_id: cve.assigner_id.value(),
            assigner_name: cve.assigner_name.value(),
            date_published: cve.date_published.value(),
            date_updated: cve.date_updated.value(),
        }
    }
}
