use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid charge {0}")]
    InvalidCharge(String),

    #[error("invalid number")]
    InvalidFloat(#[from] ParseIntError),

    #[error("invalid integer")]
    InvalidInt(#[from] ParseFloatError),

    #[error("unknown entry {0}")]
    UnknownEntry(String),
}
