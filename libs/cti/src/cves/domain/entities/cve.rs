use super::{cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState};


pub struct Cve {
    pub id: CveId,
    pub state: CveState,
    pub date_published: CvePublicationDate,
    
    pub description: CveDescription,
}