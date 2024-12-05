use std::ops::Deref;

use async_trait::async_trait;
use cqrs::domain::{
    query::Query, query_bus_response::QueryBusResponse, query_handler::QueryHandler,
};

use crate::cves::{
    application::cve_query_response::CveQueryResponse,
    domain::{entities::cve_id::CveId, repositories::cve_repository::CveRepository},
};

use super::{cve_finder::CveFinder, find_cve_query::FindCveQuery};

pub struct FindCveQueryHandler<R: CveRepository> {
    crypto_keys_by_user_finder: CveFinder<R>,
}

impl<R: CveRepository> FindCveQueryHandler<R> {
    pub fn new(crypto_keys_by_user_finder: CveFinder<R>) -> FindCveQueryHandler<R> {
        FindCveQueryHandler {
            crypto_keys_by_user_finder,
        }
    }
}

#[async_trait]
impl<R: CveRepository> QueryHandler for FindCveQueryHandler<R> {
    async fn handle(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse> {
        let query = query
            .deref()
            .as_any()
            .downcast_ref::<FindCveQuery>()
            .unwrap();

        let id = match CveId::from_optional(&query.id) {
            Ok(id) => id,
            Err(err) => return CveQueryResponse::boxed_err(err),
        };

        let res = self.crypto_keys_by_user_finder.run(id).await;
        if res.is_err() {
            return CveQueryResponse::boxed_err(res.err().unwrap());
        }

        CveQueryResponse::boxed_ok(res.ok().unwrap())
    }

    fn subscribet_to(&self) -> String {
        return FindCveQuery::QUERY_TYPE.to_string();
    }
}
