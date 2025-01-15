use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::domain::{
    query::Query, query_bus::QueryBus, query_bus_response::QueryBusResponse,
    query_handler::QueryHandler,
};

pub struct InMemoryQueryBus {
    handlers: RwLock<HashMap<String, Arc<dyn QueryHandler>>>,
}

impl InMemoryQueryBus {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl QueryBus for InMemoryQueryBus {
    async fn register(&self, handler: Arc<dyn QueryHandler>) {
        self.handlers.write().await
            .insert(handler.subscribet_to().to_string(), handler);
    }

    async fn ask(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse> {
        let handlers = self.handlers.read().await;
        let handler = handlers.get(&query.get_type());
        if handler.is_none() {
            panic!("No handler found for query: {}", query.get_type());
        }
        let handler = handler.unwrap();
        handler.handle(query).await
    }
}
