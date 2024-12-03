use aggregate_root::domain::aggregate_root::AggregateRoot;

use super::{cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState};


pub struct Cve<'a> {
    pub id: &'a CveId,
    pub state: CveState,
    pub date_published: CvePublicationDate,
    
    pub description: CveDescription,
}

impl<'a> Cve<'a> {
    pub fn new(
        id: &'a CveId, 
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

impl<'a> AggregateRoot for Cve<'a> {
    fn get_type() -> String {
        "com.tanukibox.tanuki-library.cti.cve".to_string()
    }
}

impl<'a> Clone for Cve<'a> {
    fn clone(&self) -> Self {
        Self::new(
            self.id,
            self.state.clone(),
            self.date_published.clone(),
            self.description.clone(),
        )
    }
}