use std::sync::Arc;

use super::{command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler};



pub trait CommandBus {
    fn register(&mut self, handler: Arc<dyn CommandHandler>);
    fn dispatch(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse>;
}