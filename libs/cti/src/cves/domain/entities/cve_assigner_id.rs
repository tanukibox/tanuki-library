use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CveAssignerId {
    value: String,
}

impl CveAssignerId {
    pub fn new(value: &String) -> Result<Self, DomainError> {
        Ok(Self {
            value: value.clone(),
        })
    }

    pub fn from_optional(value: &Option<String>) -> Result<Self, DomainError> {
        if value.is_none() {
            return Err(DomainError::ValueObjectError {
                value: "CVE state must not be null".to_string(),
            });
        }
        let value = value.as_ref().unwrap();
        Self::new(&value)
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn ref_value(&self) -> &String {
        &self.value
    }
}

impl Clone for CveAssignerId {
    fn clone(&self) -> Self {
        Self::new(&self.value).unwrap()
    }
}

impl PartialEq for CveAssignerId {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for CveAssignerId {}

impl Hash for CveAssignerId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
