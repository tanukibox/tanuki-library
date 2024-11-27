use super::{query::Query, query_bus_response::QueryBusResponse};


pub trait QueryHandler {
    fn handle(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse>;
    fn subscribet_to(&self) -> String;
}