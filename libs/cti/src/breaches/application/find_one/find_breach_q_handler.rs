use std::ops::Deref;

use async_trait::async_trait;
use cqrs::domain::{
    query::Query, query_bus_response::QueryBusResponse, query_handler::QueryHandler,
};

use crate::{breaches::{application::breach_query_response::BreachQueryResponse, domain::{entities::{breach_product::BreachProduct, breach_product_version::BreachProductVersion, breach_vendor::BreachVendor}, repositories::breach_repository::BreachRepository}}, cves::domain::entities::cve_id::CveId};

use super::{breach_finder::BreachFinder, find_breach_query::FindBreachQuery};

pub struct FindBreachQueryHandler<R: BreachRepository> {
    finder: BreachFinder<R>,
}

impl<R: BreachRepository> FindBreachQueryHandler<R> {
    pub fn new(finder: BreachFinder<R>) -> FindBreachQueryHandler<R> {
        FindBreachQueryHandler {
            finder,
        }
    }
}

#[async_trait]
impl<R: BreachRepository> QueryHandler for FindBreachQueryHandler<R> {
    async fn handle(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse> {
        let query = query
            .deref()
            .as_any()
            .downcast_ref::<FindBreachQuery>()
            .unwrap();

            let vendor = match BreachVendor::from_optional(&query.vendor) {
                Ok(vendor) => vendor,
                Err(err) => return BreachQueryResponse::boxed_err(err),
            };
    
            let product = match BreachProduct::from_optional(&query.product) {
                Ok(product) => product,
                Err(err) => return BreachQueryResponse::boxed_err(err),
            };
    
            let product_version = match BreachProductVersion::from_optional(&query.version) {
                Ok(product_version) => product_version,
                Err(err) => return BreachQueryResponse::boxed_err(err),
            };
    
            let cve_id = match CveId::from_optional(&query.cve_id) {
                Ok(id) => id,
                Err(err) => return BreachQueryResponse::boxed_err(err),
            };

        let res = self.finder.run(vendor, product, product_version, cve_id).await;
        if res.is_err() {
            return BreachQueryResponse::boxed_err(res.err().unwrap());
        }

        BreachQueryResponse::boxed_ok(res.ok().unwrap())
    }

    fn subscribet_to(&self) -> String {
        return FindBreachQuery::QUERY_TYPE.to_string();
    }
}
