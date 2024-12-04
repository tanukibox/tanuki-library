use cqrs::domain::command::Command;


pub struct DeleteCveCommand {
    pub id: Option<String>,
}

impl DeleteCveCommand {
    pub const COMMAND_TYPE: &'static str = "com.tanukibox.cti.cves.delete-one@1.0.0";

    pub fn new(id: Option<String>) -> DeleteCveCommand {
        DeleteCveCommand { id }
    }
}

impl Command for DeleteCveCommand {
    fn command_type(&self) -> String {
        DeleteCveCommand::COMMAND_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}