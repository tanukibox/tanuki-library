use std::any::Any;

use cqrs::domain::query::Query;

pub struct FindBreachQuery {
    pub cve_id: Option<String>,
    pub vendor: Option<String>,
    pub product: Option<String>,
    pub version: Option<String>,
}

impl FindBreachQuery {
    pub const QUERY_TYPE: &'static str = "FindBreachQuery";

    pub fn new(
        cve_id: Option<String>,
        vendor: Option<String>,
        product: Option<String>,
        version: Option<String>,
    ) -> FindBreachQuery {
        FindBreachQuery {
            cve_id,
            vendor,
            product,
            version,
        }
    }

    pub fn new_boxed(
        cve_id: Option<String>,
        vendor: Option<String>,
        product: Option<String>,
        version: Option<String>,
    ) -> Box<dyn Query> {
        Box::new(FindBreachQuery::new(cve_id, vendor, product, version))
    }
}

impl Query for FindBreachQuery {
    fn get_type(&self) -> String {
        FindBreachQuery::QUERY_TYPE.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
