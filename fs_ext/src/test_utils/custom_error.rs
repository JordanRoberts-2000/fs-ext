use std::{error, fmt};

#[derive(Debug)]
pub struct CustomError(pub &'static str);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}
impl error::Error for CustomError {}
