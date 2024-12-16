use crate::breaches::domain::entities::breach_product::BreachProduct;
use crate::breaches::domain::entities::breach_product_type::BreachProductType;
use crate::breaches::domain::entities::breach_product_version::BreachProductVersion;
use crate::breaches::domain::entities::breach_vendor::BreachVendor;
use crate::breaches::domain::entities::{breach::Breach, breach_id::BreachId};
use crate::cves::domain::entities::cve_assigner_id::CveAssignerId;
use crate::cves::domain::entities::cve_assigner_name::CveAssignerName;
use crate::cves::domain::entities::cve_description::CveDescription;
use crate::cves::domain::entities::cve_id::CveId;
use crate::cves::domain::entities::cve_publication_date::CvePublicationDate;
use crate::cves::domain::entities::cve_state::CveState;
use crate::cves::domain::entities::cve_updated_date::CveUpdatedDate;
use crate::shared::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BreachJsonDto {
    pub id: Option<String>,

    pub vendor: Option<String>,
    pub product: Option<String>,
    pub product_version: Option<String>,
    pub product_type: Option<String>,

    pub cve_id: Option<String>,
    pub cve_state: Option<String>,
    pub cve_description: Option<String>,
    pub cve_assigner_id: Option<String>,
    pub cve_assigner_name: Option<String>,

    pub cve_date_published: Option<String>,
    pub cve_date_updated: Option<String>,
}

pub fn parse_to_dto(breach: &Breach) -> BreachJsonDto {
    BreachJsonDto {
        id: Some(breach.id.value()),
        vendor: Some(breach.vendor.value()),
        product: Some(breach.product.value()),
        product_version: Some(breach.product_version.value()),
        product_type: Some(breach.product_type.value()),
        cve_id: Some(breach.cve_id.value()),
        cve_state: Some(breach.cve_state.value()),
        cve_description: breach.cve_description.value(),
        cve_assigner_id: Some(breach.cve_assigner_id.value()),
        cve_assigner_name: Some(breach.cve_assigner_name.value()),

        cve_date_published: Some(breach.cve_date_published.value()),
        cve_date_updated: Some(breach.cve_date_updated.value()),
    }
}

pub fn parse_to_domain(dto: &BreachJsonDto) -> Result<Breach, DomainError> {
    let id = BreachId::from_optional(&dto.id)
        .or_else(|e| Err(e))
        .unwrap();

    let vendor = BreachVendor::from_optional(&dto.vendor)
        .or_else(|e| Err(e))
        .unwrap();

    let product = BreachProduct::from_optional(&dto.product)
        .or_else(|e| Err(e))
        .unwrap();

    let product_version = BreachProductVersion::from_optional(&dto.product_version)
        .or_else(|e| Err(e))
        .unwrap();

    let product_type = BreachProductType::from_optional(&dto.product_type)
        .or_else(|e| Err(e))
        .unwrap();

    let cve_id = CveId::from_optional(&dto.cve_id)
        .or_else(|e| Err(e))
        .unwrap();

    let cve_state = CveState::from_optional(&dto.cve_state)
        .or_else(|e| Err(e))
        .unwrap();

    let cve_description = CveDescription::new(&dto.cve_description)
        .or_else(|e| Err(e))
        .unwrap();

    let cve_assigner_id = CveAssignerId::from_optional(&dto.cve_assigner_id)
        .or_else(|e| Err(e))
        .unwrap();

    let cve_assigner_name = CveAssignerName::from_optional(&dto.cve_assigner_name)
        .or_else(|e| Err(e))
        .unwrap();

    let cve_date_updated = CveUpdatedDate::from_optional(&dto.cve_date_updated)
        .or_else(|e| Err(e))
        .unwrap();

    let cve_date_published = CvePublicationDate::from_optional(&dto.cve_date_published)
        .or_else(|e| Err(e))
        .unwrap();

    let breach = Breach::from(
        &id,
        &vendor,
        &product,
        &product_version,
        &product_type,
        &cve_id,
        &cve_state,
        &cve_description,
        &cve_assigner_id,
        &cve_assigner_name,
        &cve_date_published,
        &cve_date_updated,
    );

    Ok(breach)
}
