use std::error::Error;

pub type AocResult<T> = Result<T, AocError>;
pub type AocError = Box<dyn Error + 'static>;
