use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CveDescription {
    value: String,
}

impl CveDescription {
    pub fn new(value: String) -> Result<Self, DomainError> {
        Ok(Self { value })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CveDescription {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
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
