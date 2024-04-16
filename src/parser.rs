use crate::error::{ParseError, ParseError::*};

use crate::{AppliedItem, Token};
use crate::AbstractItem::Applied;
use crate::scanner::{Lexeme, Lexeme::*};
use crate::Token::Abstract;


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