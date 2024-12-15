use crate::{
    breaches::domain::entities::{
        breach::Breach, breach_product::BreachProduct,
        breach_product_version::BreachProductVersion, breach_vendor::BreachVendor,
    },
    cves::domain::entities::cve_id::CveId,
    shared::domain::errors::DomainError,
};
use async_trait::async_trait;

#[async_trait]
pub trait BreachRepository: Send + Sync + 'static {
    async fn find_one(
        &self,
        cve_id: &CveId,
        vendor: BreachVendor,
        product: BreachProduct,
        product_version: BreachProductVersion,
    ) -> Result<Breach, DomainError>;
    async fn create_one(&self, breach: &Breach) -> Result<(), DomainError>;
    async fn update_one(&self, breach: &Breach) -> Result<(), DomainError>;
    async fn delete_one(
        &self,
        cve_id: &CveId,
        vendor: BreachVendor,
        product: BreachProduct,
        product_version: BreachProductVersion,
    ) -> Result<(), DomainError>;
}
