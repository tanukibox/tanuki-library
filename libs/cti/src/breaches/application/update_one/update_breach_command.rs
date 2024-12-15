use cqrs::domain::command::Command;

pub struct UpdateBreachCommand {
    pub id: Option<String>,

    pub vendor: Option<String>,
    pub product: Option<String>,
    pub product_version: Option<String>,
    pub old_product_type: Option<String>,
    pub product_type: Option<String>,

    pub cve_id: Option<String>,
    pub old_cve_state: Option<String>,
    pub cve_state: Option<String>,
    pub old_cve_description: Option<String>,
    pub cve_description: Option<String>,
    pub old_cve_assigner_id: Option<String>,
    pub cve_assigner_id: Option<String>,
    pub old_cve_assigner_name: Option<String>,
    pub cve_assigner_name: Option<String>,
    pub old_cve_date_published: Option<String>,
    pub cve_date_published: Option<String>,
    pub old_cve_date_updated: Option<String>,
    pub cve_date_updated: Option<String>,
}

impl UpdateBreachCommand {
    pub const COMMAND_TYPE: &'static str = "com.tanukibox.cti.breach.update-one@1.0.0";

    pub fn new(
        id: Option<String>,
        vendor: Option<String>,
        product: Option<String>,
        product_version: Option<String>,
        product_type: Option<String>,
        old_product_type: Option<String>,
        cve_id: Option<String>,
        old_cve_state: Option<String>,
        cve_state: Option<String>,
        old_cve_description: Option<String>,
        cve_description: Option<String>,
        old_cve_assigner_id: Option<String>,
        cve_assigner_id: Option<String>,
        old_cve_assigner_name: Option<String>,
        cve_assigner_name: Option<String>,
        old_cve_date_published: Option<String>,
        cve_date_published: Option<String>,
        old_cve_date_updated: Option<String>,
        cve_date_updated: Option<String>,
    ) -> UpdateBreachCommand {
        UpdateBreachCommand {
            id,
            vendor,
            product,
            product_version,
            old_product_type,
            product_type,
            cve_id,
            old_cve_state,
            cve_state,
            old_cve_description,
            cve_description,
            old_cve_assigner_id,
            cve_assigner_id,
            old_cve_assigner_name,
            cve_assigner_name,
            old_cve_date_published,
            cve_date_published,
            old_cve_date_updated,
            cve_date_updated,
        }
    }

    pub fn new_boxed(
        id: Option<String>,
        vendor: Option<String>,
        product: Option<String>,
        product_version: Option<String>,
        product_type: Option<String>,
        old_product_type: Option<String>,
        cve_id: Option<String>,
        old_cve_state: Option<String>,
        cve_state: Option<String>,
        old_cve_description: Option<String>,
        cve_description: Option<String>,
        old_cve_assigner_id: Option<String>,
        cve_assigner_id: Option<String>,
        old_cve_assigner_name: Option<String>,
        cve_assigner_name: Option<String>,
        old_cve_date_published: Option<String>,
        cve_date_published: Option<String>,
        old_cve_date_updated: Option<String>,
        cve_date_updated: Option<String>,
    ) -> Box<dyn Command> {
        Box::new(UpdateBreachCommand::new(
            id,
            vendor,
            product,
            product_version,
            product_type,
            old_product_type,
            cve_id,
            old_cve_state,
            cve_state,
            old_cve_description,
            cve_description,
            old_cve_assigner_id,
            cve_assigner_id,
            old_cve_assigner_name,
            cve_assigner_name,
            old_cve_date_published,
            cve_date_published,
            old_cve_date_updated,
            cve_date_updated,
        ))
    }
}

impl Command for UpdateBreachCommand {
    fn command_type(&self) -> String {
        UpdateBreachCommand::COMMAND_TYPE.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}