use crate::{
    breaches::domain::{entities::{breach::Breach, breach_product::BreachProduct, breach_product_version::BreachProductVersion, breach_vendor::BreachVendor}, repositories::breach_repository::BreachRepository}, cves::domain::entities::cve_id::CveId, shared::domain::errors::DomainError
};
use async_trait::async_trait;

use tracing::error;

use super::sqlx_breach::SqlxBreach;

pub struct SqlxPostgresBreachRepository {
    pool: sqlx::PgPool,
}

impl SqlxPostgresBreachRepository {
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
            .await
            .expect("Schema kernel not found.");
        Self::new(pool)
    }
}

#[async_trait]
impl BreachRepository for SqlxPostgresBreachRepository {
    async fn find_one(
        &self,
        cve_id: &CveId,
        vendor: &BreachVendor,
        product: &BreachProduct,
        product_version: &BreachProductVersion,
    ) -> Result<Breach, DomainError> {
        let query = "SELECT * FROM cti.breaches WHERE id = $1";
        let query = sqlx::query_as(query).bind(id.value());
        let key_res: Result<SqlxBreach, sqlx::Error> = query.fetch_one(&self.pool).await;
        if key_res.is_err() {
            return match key_res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::CveNotFound { id: id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            };
        }
        Ok(key_res.unwrap().to_domain())
    }

    async fn create_one(&self, cve: &Breach) -> Result<(), DomainError> {
        let sql_cve: SqlxBreach = SqlxBreach::from_domain(cve);
        let query =
            "INSERT INTO cti.breaches (id, state, description, assigner_id, assigner_name, date_published, date_updated) VALUES ($1, $2, $3, $4, $5, $6, $7)";
        let res = sqlx::query(query)
            .bind(&sql_cve.id)
            .bind(&sql_cve.state)
            .bind(&sql_cve.description)
            .bind(&sql_cve.assigner_id)
            .bind(&sql_cve.assigner_name)
            .bind(&sql_cve.date_published)
            .bind(&sql_cve.date_updated)
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() {
            // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::Database(_) => {
                    Err(DomainError::CveAlreadyExists { id: cve.id.value() })
                }
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            };
        }
        Ok(())
    }

    async fn update_one(&self, cve: &Breach) -> Result<(), DomainError> {
        let sql_cve: SqlxBreach = SqlxBreach::from_domain(cve);
        let query =
            "UPDATE cti.breaches SET state = $1, description = $2, assigner_id = $3, assigner_name = $4, date_published = $5, 
            date_updated = $6 WHERE id = $7";
        let res = sqlx::query(query)
            .bind(&sql_cve.state)
            .bind(&sql_cve.description)
            .bind(&sql_cve.assigner_id)
            .bind(&sql_cve.assigner_name)
            .bind(&sql_cve.date_published)
            .bind(&sql_cve.date_updated)
            .bind(&sql_cve.id)
            .fetch_optional(&self.pool)
            .await;

        if res.is_err() {
            // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::CveNotFound { id: cve.id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            };
        }
        Ok(())
    }

    async fn delete_one(
        &self,
        cve_id: &CveId,
        vendor: &BreachVendor,
        product: &BreachProduct,
        product_version: &BreachProductVersion,
    ) -> Result<(), DomainError> {
        let query = "DELETE FROM cti.breaches WHERE id = $1";
        let res = sqlx::query(query)
            .bind(id.value())
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() {
            // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::CveNotFound { id: id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            };
        }
        Ok(())
    }
}
