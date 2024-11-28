use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CveState {
    value: String,
}

impl CveState {
    pub fn new(value: String) -> Result<Self, DomainError> {
        Ok(Self { value })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CveState {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for CveState {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for CveState {}

impl Hash for CveState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
