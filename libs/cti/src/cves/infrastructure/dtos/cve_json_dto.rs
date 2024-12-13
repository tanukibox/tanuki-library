use crate::cves::domain::entities::cve_assigner_name::CveAssignerName;
use crate::cves::domain::entities::cve_updated_date::CveUpdatedDate;
use crate::cves::domain::entities::{cve::Cve, cve_assigner_id::CveAssignerId};
use crate::cves::domain::entities::cve_description::CveDescription;
use crate::cves::domain::entities::cve_id::CveId;
use crate::cves::domain::entities::cve_publication_date::CvePublicationDate;
use crate::cves::domain::entities::cve_state::CveState;
use crate::shared::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CveJsonDto {
    pub id: Option<String>,
    pub state: Option<String>,
    pub description: Option<String>,
    pub assigner_id: Option<String>,
    pub assigner_name: Option<String>,

    pub date_published: Option<String>,
    pub date_updated: Option<String>,
}

pub fn parse_to_dto(cve: &Cve) -> CveJsonDto {
    CveJsonDto {
        id: Some(cve.id.value()),
        state: Some(cve.state.value()),
        description: cve.description.value(),
        assigner_id: Some(cve.assigner_id.value()),
        assigner_name: Some(cve.assigner_name.value()),
        
        date_published: Some(cve.date_published.value()),
        date_updated: Some(cve.date_updated.value()),
    }
}

pub fn parse_to_domain(dto: &CveJsonDto) -> Result<Cve, DomainError> {
    let id = CveId::from_optional(&dto.id).or_else(|e| Err(e)).unwrap();
    let state = CveState::from_optional(&dto.state)
        .or_else(|e| Err(e))
        .unwrap();
    let description = CveDescription::new(&dto.description)
        .or_else(|e| Err(e))
        .unwrap();
    let assigner_id = CveAssignerId::from_optional(&dto.assigner_id)
        .or_else(|e| Err(e))
        .unwrap();
    let assigner_name = CveAssignerName::from_optional(&dto.assigner_name)
        .or_else(|e| Err(e))
        .unwrap();
    let date_published = CvePublicationDate::from_optional(&dto.date_published)
        .or_else(|e| Err(e))
        .unwrap();
    let date_updated = CveUpdatedDate::from_optional(&dto.date_updated)
        .or_else(|e| Err(e))
        .unwrap();

    let cve = Cve::new(id, state, description, assigner_id, assigner_name, date_published, date_updated);

    Ok(cve)
}
