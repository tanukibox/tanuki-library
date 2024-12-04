use tracing::debug;

use crate::{cves::domain::{entities::{cve::Cve, cve_id::CveId}, repositories::cve_repository::CveRepository}, shared::domain::errors::DomainError};
use std::sync::Arc;

pub struct CveFinder<R: CveRepository> {
    repository: Arc<R>,
}

impl<R: CveRepository> CveFinder<R> {
    pub fn new(cve_repository: Arc<R>) -> CveFinder<R> {
        CveFinder { repository: cve_repository }
    }

    pub async fn run(&self, id: CveId) -> Result<Cve, DomainError> {
        debug!("Finding CVE with id: {}.", id);
        let res = self.repository.find_by_id(&id).await;
        if res.is_err() {
            debug!("Error finding CVE with id: {}.", id);
            return Err(res.err().unwrap());
        }
        debug!("CVE with id: {} found.", id);
        Ok(res.unwrap())
    }
}
