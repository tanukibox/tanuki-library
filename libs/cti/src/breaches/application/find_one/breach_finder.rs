use tracing::debug;

use crate::{
    breaches::domain::{entities::{breach::Breach, breach_product::BreachProduct, breach_product_version::BreachProductVersion, breach_vendor::BreachVendor}, repositories::breach_repository::BreachRepository}, cves::domain::entities::{cve::Cve, cve_id::CveId}, shared::domain::errors::DomainError
};
use std::sync::Arc;

pub struct BreachFinder<R: BreachRepository> {
    repository: Arc<R>,
}

impl<R: BreachRepository> BreachFinder<R> {
    pub fn new(repository: Arc<R>) -> BreachFinder<R> {
        BreachFinder {
            repository,
        }
    }

    pub async fn run(
        &self,
        vendor: BreachVendor,
        product: BreachProduct,
        product_version: BreachProductVersion,
        cve_id: CveId,
    ) -> Result<Breach, DomainError> {
        debug!("Finding Breach with CVE: {} for Product: {}:{}:{}.", cve_id, vendor, product, product_version);
        let res = self.repository.find_one(&cve_id, &vendor, &product, &product_version).await;
        if res.is_err() {
            debug!("Error finding Breach with CVE: {} for Product: {}:{}:{}.", cve_id, vendor, product, product_version);
            return Err(res.err().unwrap());
        }
        debug!("Breach with CVE: {} for Product: {}:{}:{}.", cve_id, vendor, product, product_version);
        
        return res;
    }
}
