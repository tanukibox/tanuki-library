use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CveDescription {
    value: Option<String>,
}

impl CveDescription {
    pub fn new(value: &Option<String>) -> Result<Self, DomainError> {
        Ok(Self { value: value.clone() })
    }

    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }

    pub fn ref_value(&self) -> &Option<String> {
        &self.value
    }
}

impl Clone for CveDescription {
    fn clone(&self) -> Self {
        Self::new(&self.value).unwrap()
    }
}

impl PartialEq for CveDescription {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for CveDescription {}

impl Hash for CveDescription {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
