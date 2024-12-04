use async_trait::async_trait;
use crate::{cves::domain::{entities::{cve::Cve, cve_id::CveId}, repositories::cve_repository::CveRepository}, shared::domain::errors::DomainError};

use tracing::error;

use super::sqlx_cve::SqlxCve;

pub struct SqlxPostgresCveRepository {
    pool: sqlx::PgPool,
}

impl SqlxPostgresCveRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn from_env() -> Self {
        let url_load_res = std::env::var("DATABASE_URL");
        if url_load_res.is_err() {
            panic!("DATABASE_URL not found in environment variables.");
        }
        let url = url_load_res.unwrap();
        let pool_res = sqlx::PgPool::connect(&url).await;
        if pool_res.is_err() {
            panic!("Failed to connect to database: {:?}", pool_res.err());
        }
        let pool = pool_res.unwrap();
        sqlx::query("SET search_path TO kernel")
            .execute(&pool)
            .await.expect("Schema kernel not found.");
        Self::new(pool)
    }
}

#[async_trait]
impl CveRepository for SqlxPostgresCveRepository {

    async fn find_by_id(&self, id: &CveId) -> Result<Cve, DomainError> {
        let query = "SELECT * FROM tanukilibrary.cves WHERE id = $1";
        let query = sqlx::query_as(query)
            .bind(id.value());
        let key_res: Result<SqlxCve, sqlx::Error> = query.fetch_one(&self.pool).await;
        if key_res.is_err() {
            return match key_res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::CveNotFound { id: id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(key_res.unwrap().to_domain())
    }

    async fn create_one(&self, cve: &Cve) -> Result<(), DomainError> {
        let sql_cve: SqlxCve = SqlxCve::from_domain(cve);
        let query = "INSERT INTO tanukilibrary.cves (id, state, date_published, description) VALUES ($1, $2, $3, $4)";
        let res = sqlx::query(query)
            .bind(&sql_cve.id)
            .bind(&sql_cve.state)
            .bind(&sql_cve.date_published)
            .bind(&sql_cve.description)
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() { // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::Database(_) => Err(DomainError::CveAlreadyExists { id: cve.id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(())
    }

    async fn update_one(&self, cve: &Cve) -> Result<(), DomainError> {
        let sql_cve: SqlxCve = SqlxCve::from_domain(cve);
        let query = "UPDATE tanukilibrary.cves SET state = $1, date_published = $2, description = $3 WHERE id = $3";
        let res = sqlx::query(query)
            .bind(&sql_cve.state)
            .bind(&sql_cve.date_published)
            .bind(&sql_cve.description)
            .bind(&sql_cve.id)
            .fetch_optional(&self.pool)
            .await;

        if res.is_err() { // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::CveNotFound { id: cve.id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(())
    }

    async fn delete_one(&self, id: &CveId) -> Result<(), DomainError> {
        let query = "DELETE FROM tanukilibrary.cves WHERE id = $1";
        let res = sqlx::query(query)
            .bind(id.value())
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() { // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::CveNotFound { id: id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(())
    }
}

