use aggregate_root::domain::aggregate_root::AggregateRoot;

use super::{cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState};


pub struct Cve {
    pub id: CveId,
    pub state: CveState,
    pub date_published: CvePublicationDate,
    
    pub description: CveDescription,
}

impl Cve {
    pub fn new(
        id: CveId, 
        state: CveState, 
        date_published: CvePublicationDate, 
        description: CveDescription
    ) -> Self {
        Self {
            id,
            state,
            date_published,
            description,
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
        Self::new(
            self.id.clone(),
            self.state.clone(),
            self.date_published.clone(),
            self.description.clone(),
        )
    }
}