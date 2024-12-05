use crate::{
    cves::domain::entities::{cve::Cve, cve_id::CveId},
    shared::domain::errors::DomainError,
};
use async_trait::async_trait;

#[async_trait]
pub trait CveRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: &CveId) -> Result<Cve, DomainError>;
    async fn create_one(&self, user: &Cve) -> Result<(), DomainError>;
    async fn update_one(&self, user: &Cve) -> Result<(), DomainError>;
    async fn delete_one(&self, id: &CveId) -> Result<(), DomainError>;
}
