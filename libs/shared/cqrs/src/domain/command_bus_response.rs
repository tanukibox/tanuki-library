
pub trait CommandBusResponse {
    fn response_type(&self) -> String;
}