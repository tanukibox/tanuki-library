use crate::{
    breaches::domain::entities::{breach::Breach, breach_id::BreachId},
    shared::domain::errors::DomainError,
};
use async_trait::async_trait;

#[async_trait]
pub trait BreachRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: &BreachId) -> Result<Breach, DomainError>;
    async fn create_one(&self, breach: &Breach) -> Result<(), DomainError>;
    async fn update_one(&self, breach: &Breach) -> Result<(), DomainError>;
    async fn delete_one(&self, id: &BreachId) -> Result<(), DomainError>;
}
