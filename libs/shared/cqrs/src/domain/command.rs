
pub trait Command {
    fn command_type(&self) -> String;
    fn as_any (&self) -> &dyn std::any::Any;
}