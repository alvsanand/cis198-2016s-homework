use std::error::Error;

pub type GenericError = Box<dyn Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, GenericError>;
