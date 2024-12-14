use events::domain::domain_event::DomainEvent;

use crate::{
    breaches::domain::entities::{
        breach_id::BreachId, breach_product::BreachProduct, breach_product_type::BreachProductType,
        breach_product_version::BreachProductVersion, breach_vendor::BreachVendor,
    },
    cves::domain::entities::{
        cve_assigner_id::CveAssignerId, cve_assigner_name::CveAssignerName,
        cve_description::CveDescription, cve_id::CveId, cve_publication_date::CvePublicationDate,
        cve_state::CveState, cve_updated_date::CveUpdatedDate,
    },
};

pub struct BreachCreatedEvent {
    pub id: String,

    // Breach data
    pub breach_id: BreachId,

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

    pub occurred_on: String,
}

impl BreachCreatedEvent {
    pub fn new(
        breach_id: &BreachId,
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
        let id = uuid::Uuid::new_v4().to_string();
        let occurred_on = chrono::Utc::now().to_rfc3339();
        Self {
            id,
            breach_id: breach_id.clone(),
            vendor: vendor.clone(),
            product: product.clone(),
            product_version: product_version.clone(),
            product_type: product_type.clone(),
            cve_id: cve_id.clone(),
            cve_state: cve_state.clone(),
            cve_description: cve_description.clone(),
            cve_assigner_id: cve_assigner_id.clone(),
            cve_assigner_name: cve_assigner_name.clone(),
            cve_date_published: cve_date_published.clone(),
            cve_date_updated: cve_date_updated.clone(),
            occurred_on,
        }
    }

    pub fn new_shared(
        breach_id: &BreachId,
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
    ) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self::new(
            breach_id,
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
        ))
    }
}

impl DomainEvent for BreachCreatedEvent {
    fn event_type(&self) -> String {
        "com.tanukibox.cti.cve.created@1.0.0".to_string()
    }
}

impl Clone for BreachCreatedEvent {
    fn clone(&self) -> Self {
        let mut event = Self::new(
            &self.breach_id,
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
        );
        event.occurred_on = self.occurred_on.clone();
        return event;
    }
}
