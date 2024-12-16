use cqrs::domain::command::Command;

pub struct CreateBreachCommand {
    pub id: Option<String>,

    pub vendor: Option<String>,
    pub product: Option<String>,
    pub product_version: Option<String>,
    pub product_type: Option<String>,

    pub cve_id: Option<String>,
}

impl CreateBreachCommand {
    pub const COMMAND_TYPE: &'static str = "com.tanukibox.cti.breach.create-one@1.0.0";

    pub fn new(
        id: Option<String>,
        vendor: Option<String>,
        product: Option<String>,
        product_version: Option<String>,
        product_type: Option<String>,
        cve_id: Option<String>,
    ) -> CreateBreachCommand {
        CreateBreachCommand {
            id,
            vendor,
            product,
            product_version,
            product_type,
            cve_id,
        }
    }

    pub fn new_boxed(
        id: Option<String>,
        vendor: Option<String>,
        product: Option<String>,
        product_version: Option<String>,
        product_type: Option<String>,
        cve_id: Option<String>,
    ) -> Box<dyn Command> {
        Box::new(CreateBreachCommand::new(
            id,
            vendor,
            product,
            product_version,
            product_type,
            cve_id,
        ))
    }
}

impl Command for CreateBreachCommand {
    fn command_type(&self) -> String {
        CreateBreachCommand::COMMAND_TYPE.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
