use std::sync::Arc;

use super::{query::Query, query_bus_response::QueryBusResponse, query_handler::QueryHandler};



pub trait QueryBus {
    fn register(&mut self, handler: Arc<dyn QueryHandler>);
    fn ask(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse>;
}