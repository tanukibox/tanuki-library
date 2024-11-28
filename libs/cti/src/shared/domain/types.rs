use std::error;

pub type DynError = Box<dyn error::Error>;