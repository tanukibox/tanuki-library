use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CveId {
    value: String,
}

impl CveId {
    pub fn new(id: &String) -> Result<Self, DomainError> {
        if id.contains(" ") {
            return Err(DomainError::ValueObjectError { value: "CVE id must not contain blank spaces".to_string() })
        }
        Ok(Self { value: id.clone() })
    }

    pub fn from_optional(id: &Option<String>) -> Result<Self, DomainError> {
        if id.is_none() {
            return Err(DomainError::ValueObjectError { value: "CVE id must not be empty".to_string() })
        }
        let id = id.as_ref().unwrap();
        Self::new(&id)
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn rvalue(&self) -> &String {
        &self.value
    }
}

impl Clone for CveId {
    fn clone(&self) -> Self {
        Self::new(&self.value).unwrap()
    }
}

impl PartialEq for CveId {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for CveId {}

impl Hash for CveId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl std::fmt::Display for CveId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
