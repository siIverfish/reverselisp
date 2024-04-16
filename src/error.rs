use std::num::ParseIntError;
use crate::{AbstractItem, DataItem, Token};

pub use {ScanError::*, ParseError::*, EvalError::*, Error::*};

#[derive(thiserror::Error, Debug)]
pub enum ScanError {
    #[error("malformed number: {0}")]
    MalformedNumber(#[from] ParseIntError),
    #[error("could not read input")]
    IOError(#[from] std::io::Error),
    #[error("unexpected EOF")]
    EOF,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("empty program segment")]
    EndOfInput,
    #[error("parser encountered unexpected whitespace token -- there is a bug in the scanner.")]
    UnexpectedWhitespace,
}

#[derive(thiserror::Error, Debug)]
pub enum EvalError {
    #[error("Token {0:?} could not be coerced into the wanted type.")]
    NotAValue(Token),
    #[error("meta fragment {0:?} cannot be called.")]
    MetaFragment(AbstractItem),
    #[error("Type mismatch: `{s:?}` is not `{t}`")]
    TypeMismatch { s: Token, t: &'static str},
    #[error("Runtime error: {0})")]
    RuntimeError(String),
    #[error("not an application: '{0:?}'")]
    EvaluateFunctionError(Token),
    #[error("Token {token:?} could not be coerced into {wrong_type:?}")]
    WrongType { token: DataItem, wrong_type: String },
    #[error("ident {0} unknown")]
    CouldNotResolve(String),
    #[error("Could not get value from non-abstract token {0:?}")]
    CouldNotGet(Token),
    #[error("could not destructure token {0:?}")]
    CouldNotDestructure(Token),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error while parsing: {0}")]
    Parse(#[from] ParseError),
    #[error("error while scanning: {0}")]
    Scan(#[from] ScanError),
    #[error("error during evaluation: {0}")]
    Eval(#[from] EvalError),
}