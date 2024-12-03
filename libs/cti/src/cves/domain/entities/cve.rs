use aggregate_root::domain::aggregate_root::AggregateRoot;

use super::{cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState};


pub struct Cve<'a> {
    pub id: &'a CveId,
    pub state: &'a CveState,
    pub date_published: &'a CvePublicationDate,
    pub description: &'a CveDescription,
}

impl<'a> Cve<'a> {
    pub fn new(
        id: &'a CveId, 
        state: &'a CveState, 
        date_published: &'a CvePublicationDate, 
        description: &'a CveDescription
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
            self.state,
            self.date_published,
            self.description,
        )
    }
}