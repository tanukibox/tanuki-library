use sqlx::FromRow;

use crate::{
    breaches::domain::entities::{
        breach::Breach, breach_id::BreachId, breach_product::BreachProduct,
        breach_product_type::BreachProductType, breach_product_version::BreachProductVersion,
        breach_vendor::BreachVendor,
    },
    cves::domain::entities::{
        cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName,
        cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate,
        cve_state::CveState, cve_updated_date::CveUpdatedDate,
    },
};

#[derive(Debug, FromRow, Clone)]
pub struct SqlxBreach {
    // Breach data
    pub id: String,

    // Product data
    pub vendor: String,
    pub product: String,
    pub product_version: String,
    pub product_type: String,

    // CVE data
    pub cve_id: String,
    pub cve_state: String,
    pub cve_description: Option<String>,
    pub cve_assigner_id: String,
    pub cve_assigner_name: String,
    pub cve_date_published: String,
    pub cve_date_updated: String,
}

impl SqlxBreach {
    pub fn to_domain(self) -> Breach {
        Breach::new(
            BreachId::new(&self.id).unwrap(),
            BreachVendor::new(&self.vendor).unwrap(),
            BreachProduct::new(&self.product).unwrap(),
            BreachProductVersion::new(&self.product_version).unwrap(),
            BreachProductType::new(&self.product_type).unwrap(),
            CveId::new(&self.cve_id).unwrap(),
            CveState::new(&self.cve_state).unwrap(),
            CveDescription::new(&self.cve_description).unwrap(),
            CveAssignerId::new(&self.cve_assigner_id).unwrap(),
            CveAssignerName::new(&self.cve_assigner_name).unwrap(),
            CvePublicationDate::new(&self.cve_date_published).unwrap(),
            CveUpdatedDate::new(&self.cve_date_updated).unwrap(),
        )
    }

    pub fn from_domain(breach: &Breach) -> Self {
        Self {
            id: breach.id.value(),
            vendor: breach.vendor.value(),
            product: breach.product.value(),
            product_version: breach.product_version.value(),
            product_type: breach.product_type.value(),
            cve_id: breach.cve_id.value(),
            cve_state: breach.cve_state.value(),
            cve_description: breach.cve_description.value(),
            cve_assigner_id: breach.cve_assigner_id.value(),
            cve_assigner_name: breach.cve_assigner_name.value(),
            cve_date_published: breach.cve_date_published.value(),
            cve_date_updated: breach.cve_date_updated.value(),
        }
    }
}
