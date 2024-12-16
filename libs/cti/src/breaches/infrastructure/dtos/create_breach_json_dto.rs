use serde::{Deserialize, Serialize};

use crate::breaches::domain::entities::breach::Breach;



#[derive(Clone, Serialize, Deserialize)]
pub struct CreateBreachJsonDto {
    pub id: Option<String>,

    pub vendor: Option<String>,
    pub product: Option<String>,
    pub product_version: Option<String>,
    pub product_type: Option<String>,

    pub cve_id: Option<String>,
}

pub fn parse_to_dto(breach: &Breach) -> CreateBreachJsonDto {
    CreateBreachJsonDto {
        id: Some(breach.id.value()),
        vendor: Some(breach.vendor.value()),
        product: Some(breach.product.value()),
        product_version: Some(breach.product_version.value()),
        product_type: Some(breach.product_type.value()),
        cve_id: Some(breach.cve_id.value()),
    }
}
