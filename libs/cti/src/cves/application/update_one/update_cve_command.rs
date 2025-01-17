use cqrs::domain::command::Command;

pub struct UpdateCveCommand {
    pub id: Option<String>,
    pub state: Option<String>,
    pub description: Option<String>,
    pub assigner_id: Option<String>,
    pub assigner_name: Option<String>,
    pub date_published: Option<String>,
    pub date_updated: Option<String>,
}

impl UpdateCveCommand {
    pub const COMMAND_TYPE: &'static str = "com.tanukibox.cti.cves.update-one@1.0.0";

    pub fn new(
        id: Option<String>,
        state: Option<String>,
        description: Option<String>,
        assigner_id: Option<String>,
        assigner_name: Option<String>,
        date_published: Option<String>,
        date_updated: Option<String>,
    ) -> UpdateCveCommand {
        UpdateCveCommand {
            id,
            state,
            description,
            assigner_id,
            assigner_name,
            date_published,
            date_updated,
        }
    }

    pub fn new_boxed(
        id: Option<String>,
        state: Option<String>,
        description: Option<String>,
        assigner_id: Option<String>,
        assigner_name: Option<String>,
        date_published: Option<String>,
        date_updated: Option<String>,
    ) -> Box<dyn Command> {
        Box::new(UpdateCveCommand::new(
            id,
            state,
            description,
            assigner_id,
            assigner_name,
            date_published,
            date_updated,
        ))
    }
}

impl Command for UpdateCveCommand {
    fn command_type(&self) -> String {
        UpdateCveCommand::COMMAND_TYPE.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
