
pub trait Command {
    fn command_type(&self) -> String;
}