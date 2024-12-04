use crate::cves::domain::entities::cve::Cve;
use crate::cves::domain::entities::cve_description::CveDescription;
use crate::cves::domain::entities::cve_publication_date::CvePublicationDate;
use crate::cves::domain::entities::cve_state::CveState;
use crate::cves::domain::entities::cve_id::CveId;
use crate::shared::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CveJsonDto {
    pub id: Option<String>,
    pub state: Option<String>,
    pub date_published: Option<String>,
    pub description: Option<String>,
}

pub fn parse_to_dto(cve: &Cve) -> CveJsonDto {
    CveJsonDto {
        id: Some(cve.id.value()),
        state: Some(cve.state.value()),
        date_published: Some(cve.date_published.value()),
        description: cve.description.value(),
    }
}

pub fn parse_to_domain(dto: &CveJsonDto) -> Result<Cve, DomainError> {
    let id = CveId::from_optional(&dto.id)
        .or_else(|e| Err(e)).unwrap();
    let state = CveState::from_optional(&dto.state)
        .or_else(|e| Err(e)).unwrap();
    let date_published = CvePublicationDate::from_optional(&dto.date_published)
        .or_else(|e| Err(e)).unwrap();
    let description = CveDescription::new(&dto.description)
        .or_else(|e| Err(e)).unwrap();
    
    let cve = Cve::new(&id, &state, &date_published, &description);

    Ok(cve)
}
