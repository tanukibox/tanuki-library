use aggregate_root::domain::aggregate_root::AggregateRoot;

use super::{
    cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName, cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState, cve_updated_date::CveUpdatedDate
};

pub struct Cve {
    pub id: CveId,
    pub state: CveState,
    pub description: CveDescription,
    pub assigner_id: CveAssignerId,
    pub assigner_name: CveAssignerName,
    pub date_published: CvePublicationDate,
    pub date_updated: CveUpdatedDate,
}

impl Cve {
    pub fn from(
        id: &CveId,
        state: &CveState,
        description: &CveDescription,
        assigner_id: &CveAssignerId,
        assigner_name: &CveAssignerName,
        date_published: &CvePublicationDate,
        date_updated: &CveUpdatedDate,
    ) -> Self {
        Self::new(
            id.clone(),
            state.clone(),
            description.clone(),
            assigner_id.clone(),
            assigner_name.clone(),
            date_published.clone(),
            date_updated.clone(),
        )
    }

    pub fn new(
        id: CveId,
        state: CveState,
        description: CveDescription,
        assigner_id: CveAssignerId,
        assigner_name: CveAssignerName,
        date_published: CvePublicationDate,
        date_updated: CveUpdatedDate,
    ) -> Self {
        Self {
            id,
            state,
            description,
            assigner_id,
            assigner_name,
            date_published,
            date_updated,
        }
    }
}

impl AggregateRoot for Cve {
    fn get_type() -> String {
        "com.tanukibox.tanuki-library.cti.cve".to_string()
    }
}

impl Clone for Cve {
    fn clone(&self) -> Self {
        Self::from(
            &self.id,
            &self.state,
            &self.description,
            &self.assigner_id,
            &self.assigner_name,
            &self.date_published,
            &self.date_updated,
        )
    }
}
