
pub trait QueryBusResponse {
    fn response_type(&self) -> String;
}