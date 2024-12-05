use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CvePublicationDate {
    value: String,
}

impl CvePublicationDate {
    pub fn new(value: &String) -> Result<Self, DomainError> {
        Ok(Self {
            value: value.clone(),
        })
    }

    pub fn from_optional(value: &Option<String>) -> Result<Self, DomainError> {
        if value.is_none() {
            return Err(DomainError::ValueObjectError {
                value: "CVE publication date must not be null".to_string(),
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

impl Clone for CvePublicationDate {
    fn clone(&self) -> Self {
        Self::new(&self.value).unwrap()
    }
}

impl PartialEq for CvePublicationDate {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for CvePublicationDate {}

impl Hash for CvePublicationDate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
