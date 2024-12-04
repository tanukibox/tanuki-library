use std::any::Any;

use cqrs::domain::query::Query;


pub struct FindCveQuery {
    pub id: Option<String>,
}

impl FindCveQuery {
    pub const QUERY_TYPE: &'static str = "FindCveQuery";

    pub fn new(id: Option<String>) -> FindCveQuery {
        FindCveQuery { id }
    }
}

impl Query for FindCveQuery {
    fn get_type(&self) -> String {
        FindCveQuery::QUERY_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}