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
        let query = "SELECT * FROM cti.breaches WHERE cve_id = $1, vendor = $2, product = $3, product_version = $4";
        let query = sqlx::query_as(query)
            .bind(cve_id.value())
            .bind(vendor.value())
            .bind(product.value())
            .bind(product_version.value());
        let key_res: Result<SqlxBreach, sqlx::Error> = query.fetch_one(&self.pool).await;
        if key_res.is_err() {
            return match key_res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::BreachNotFound { 
                    cve_id: cve_id.value(),
                    vendor: vendor.value(),
                    product: product.value(),
                    product_version: product_version.value(),
                }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            };
        }
        Ok(key_res.unwrap().to_domain())
    }

    async fn create_one(&self, breach: &Breach) -> Result<(), DomainError> {
        let sql_breach: SqlxBreach = SqlxBreach::from_domain(breach);
        let query =
            "INSERT INTO cti.breaches (id, vendor, product, product_version, product_type, cve_id, cve_state,
            cve_description, cve_assigner_id, cve_assigner_name, cve_date_published, cve_date_updated) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)";
        let res = sqlx::query(query)
            .bind(&sql_breach.id)
            .bind(&sql_breach.vendor)
            .bind(&sql_breach.product)
            .bind(&sql_breach.product_version)
            .bind(&sql_breach.product_type)
            .bind(&sql_breach.cve_id)
            .bind(&sql_breach.cve_state)
            .bind(&sql_breach.cve_description)
            .bind(&sql_breach.cve_assigner_id)
            .bind(&sql_breach.cve_assigner_name)
            .bind(&sql_breach.cve_date_published)
            .bind(&sql_breach.cve_date_updated)
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() {
            // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::Database(_) => {
                    Err(DomainError::BreachNotFound { 
                        cve_id: breach.cve_id.value(),
                        vendor: breach.vendor.value(),
                        product: breach.product.value(),
                        product_version: breach.product_version.value(),
                    })
                }
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            };
        }
        Ok(())
    }

    async fn update_one(&self, breach: &Breach) -> Result<(), DomainError> {
        let sql_breach: SqlxBreach = SqlxBreach::from_domain(breach);
        let query =
            "UPDATE cti.breaches SET product_type = $1, cve_state = $2, cve_description = $3, cve_assigner_id = $4, cve_assigner_name = $5, 
            cve_date_published = $6, cve_date_updated = $7 WHERE cve_id = $7, vendor = $8, product = $9, product_version = $10";
        let res = sqlx::query(query)
            .bind(&sql_breach.product_type)
            .bind(&sql_breach.cve_state)
            .bind(&sql_breach.cve_description)
            .bind(&sql_breach.cve_assigner_id)
            .bind(&sql_breach.cve_assigner_name)
            .bind(&sql_breach.cve_date_published)
            .bind(&sql_breach.cve_date_updated)
            .bind(&sql_breach.cve_id)
            .bind(&sql_breach.vendor)
            .bind(&sql_breach.product)
            .bind(&sql_breach.product_version)
            .fetch_optional(&self.pool)
            .await;

        if res.is_err() {
            // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::BreachNotFound {
                    cve_id: breach.cve_id.value(),
                    vendor: breach.vendor.value(),
                    product: breach.product.value(),
                    product_version: breach.product_version.value(),
                }),
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
        vendor: BreachVendor,
        product: BreachProduct,
        product_version: BreachProductVersion,
    ) -> Result<(), DomainError> {
        let query = "DELETE FROM cti.breaches WHERE cve_id = $1, vendor = $2, product = $3, product_version = $4";
        let res = sqlx::query(query)
            .bind(cve_id.value())
            .bind(vendor.value())
            .bind(product.value())
            .bind(product_version.value())
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() {
            // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::BreachNotFound {
                    cve_id: cve_id.value(),
                    vendor: vendor.value(),
                    product: product.value(),
                    product_version: product_version.value(),
                }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            };
        }
        Ok(())
    }
}
