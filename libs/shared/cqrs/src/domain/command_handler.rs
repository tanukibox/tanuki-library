use super::{command::Command, command_bus_response::CommandBusResponse};


pub trait CommandHandler {
    fn handle(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse>;
    fn subscribet_to(&self) -> String;
}