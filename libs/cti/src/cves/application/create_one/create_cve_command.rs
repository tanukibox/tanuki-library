use cqrs::domain::command::Command;


pub struct CreateCveCommand {
    pub id: Option<String>,
    pub state: Option<String>,
    pub date_published: Option<String>,
    pub description: Option<String>,
}

impl CreateCveCommand {
    pub const COMMAND_TYPE: &'static str = "com.tanukibox.cti.cves.create-one@1.0.0";

    pub fn new(id: Option<String>, state: Option<String>, date_published: Option<String>, description: Option<String>) -> CreateCveCommand {
        CreateCveCommand {
            id,
            state,
            date_published,
            description,
        }
    }
}

impl Command for CreateCveCommand {
    fn command_type(&self) -> String {
        CreateCveCommand::COMMAND_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}