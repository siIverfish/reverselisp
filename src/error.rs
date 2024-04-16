use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("malformed number: {0}")]
    MalformedNumber(#[from] ParseIntError),
    #[error("unexpected EOF")]
    EOF,
}