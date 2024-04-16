use ParseError::*;

use crate::{AppliedItem, Token};
use crate::AbstractItem::Applied;
use crate::scanner::{Lexeme, Lexeme::*};
use crate::Token::Abstract;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("empty program segment")]
    EndOfInput,
    #[error("parser encountered unexpected whitespace token -- there is a bug in the scanner.")]
    UnexpectedWhitespace,
}
// pub fn apply_result(f: Token, to: Result<Token, ParseError>) -> Result<Token, ParseError> {
//     match to {
//         Ok(to) => Ok(Abstract(Applied(Box::new(AppliedItem { f, to  })))),
//         Err(EndOfInput) => Ok(f),
//         err => err
//     }
// }
// pub fn parse_right_associative(lexemes: &mut impl Iterator<Item = Lexeme>) -> Result<Token, ParseError> {
//     match lexemes.next() {
//         Some(Start) => apply(parse_right_associative(lexemes)?, parse_right_associative(lexemes)),
//         Some(Token(t)) => apply(t, parse_right_associative(lexemes)),
//         Some(End) | None => Err(EndOfInput),
//         
//         Some(Whitespace) => Err(UnexpectedWhitespace),
//     }
// }

pub fn apply(f: Token, to: Token) -> Token {
    Abstract(Applied(Box::new(AppliedItem { f, to  })))
}


pub fn parse_literal(lexemes: &mut impl Iterator<Item = Lexeme>) -> Result<Token, ParseError> {
    match lexemes.next() {
        Some(Token(t)) => Ok(t),
        Some(Start) => parse(lexemes),
        Some(End) | None => Err(EndOfInput),
        
        Some(Whitespace) => Err(UnexpectedWhitespace),
    }
}

pub fn parse(lexemes: &mut impl Iterator<Item = Lexeme>) -> Result<Token, ParseError> {
    // try_reduce one day ??
    let mut f = parse_literal(lexemes)?;

    loop {
        match parse_literal(lexemes) {
            Ok(to) => f = apply(f, to),
            Err(EndOfInput) => break Ok(f),
            other => break other,
        }
    }
}

// pub fn parse(lexemes: &mut impl Iterator<Item = Lexeme>) -> Result<Token, ParseError>