use aggregate_root::domain::aggregate_root::AggregateRoot;

use crate::cves::domain::entities::{cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName, cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate, cve_state::CveState, cve_updated_date::CveUpdatedDate};

use super::{breach_id::BreachId, breach_product::BreachProduct, breach_product_type::BreachProductType, breach_product_version::BreachProductVersion, breach_vendor::BreachVendor};

pub struct Breach {
    // Breach data
    pub id: BreachId,

    // Product data
    pub vendor: BreachVendor,
    pub product: BreachProduct,
    pub product_version: BreachProductVersion,
    pub product_type: BreachProductType,

    // CVE data
    pub cve_id: CveId,
    pub cve_state: CveState,
    pub cve_description: CveDescription,
    pub cve_assigner_id: CveAssignerId,
    pub cve_assigner_name: CveAssignerName,
    pub cve_date_published: CvePublicationDate,
    pub cve_date_updated: CveUpdatedDate,
}

impl Breach {
    pub fn from(
        id: &BreachId,
        vendor: &BreachVendor,
        product: &BreachProduct,
        product_version: &BreachProductVersion,
        product_type: &BreachProductType,
        cve_id: &CveId,
        cve_state: &CveState,
        cve_description: &CveDescription,
        cve_assigner_id: &CveAssignerId,
        cve_assigner_name: &CveAssignerName,
        cve_date_published: &CvePublicationDate,
        cve_date_updated: &CveUpdatedDate,
    ) -> Self {
        Self::new(
            id.clone(),
            vendor.clone(),
            product.clone(),
            product_version.clone(),
            product_type.clone(),
            cve_id.clone(),
            cve_state.clone(),
            cve_description.clone(),
            cve_assigner_id.clone(),
            cve_assigner_name.clone(),
            cve_date_published.clone(),
            cve_date_updated.clone(),
        )
    }

    pub fn new(
        id: BreachId,
        vendor: BreachVendor,
        product: BreachProduct,
        product_version: BreachProductVersion,
        product_type: BreachProductType,
        cve_id: CveId,
        cve_state: CveState,
        cve_description: CveDescription,
        cve_assigner_id: CveAssignerId,
        cve_assigner_name: CveAssignerName,
        cve_date_published: CvePublicationDate,
        cve_date_updated: CveUpdatedDate,
    ) -> Self {
        Self {
            id,
            vendor,
            product,
            product_version,
            product_type,
            cve_id,
            cve_state,
            cve_description,
            cve_assigner_id,
            cve_assigner_name,
            cve_date_published,
            cve_date_updated,
        }
    }
}

impl AggregateRoot for Breach {
    fn get_type() -> String {
        "com.tanukibox.tanuki-library.cti.cve".to_string()
    }
}

impl Clone for Breach {
    fn clone(&self) -> Self {
        Self::from(
            &self.id,
            &self.vendor,
            &self.product,
            &self.product_version,
            &self.product_type,
            &self.cve_id,
            &self.cve_state,
            &self.cve_description,
            &self.cve_assigner_id,
            &self.cve_assigner_name,
            &self.cve_date_published,
            &self.cve_date_updated,
        )
    }
}

impl PartialEq for Breach {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.vendor == other.vendor
            && self.product == other.product
            && self.product_version == other.product_version
            && self.product_type == other.product_type
            && self.cve_id == other.cve_id
            && self.cve_state == other.cve_state
            && self.cve_description == other.cve_description
            && self.cve_assigner_id == other.cve_assigner_id
            && self.cve_assigner_name == other.cve_assigner_name
            && self.cve_date_published == other.cve_date_published
            && self.cve_date_updated == other.cve_date_updated
    }
}
