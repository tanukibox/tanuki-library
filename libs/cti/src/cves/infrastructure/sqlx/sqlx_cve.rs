use sqlx::FromRow;

use crate::cves::domain::entities::{
    cve::Cve, cve_description::CveDescription, cve_id::CveId,
    cve_publication_date::CvePublicationDate, cve_state::CveState,
};

#[derive(Debug, FromRow, Clone)]
pub struct SqlxCve {
    pub id: String,
    pub state: String,
    pub date_published: String,
    pub description: Option<String>,
}

impl SqlxCve {
    pub fn to_domain(self) -> Cve {
        Cve::new(
            CveId::new(&self.id).unwrap(),
            CveState::new(&self.state).unwrap(),
            CvePublicationDate::new(&self.date_published).unwrap(),
            CveDescription::new(&self.description).unwrap(),
        )
    }

    pub fn from_domain(cve: &Cve) -> Self {
        Self {
            id: cve.id.value(),
            state: cve.state.value(),
            date_published: cve.date_published.value(),
            description: cve.description.value(),
        }
    }
}
