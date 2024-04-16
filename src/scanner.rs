use crate::default_types::*;
use crate::error::ScanError::*;


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Lexeme {
    Token(crate::Token),
    Start,
    End,
    Whitespace,
}
use Lexeme::*;
use crate::builtins::Builtins;
use crate::error::ScanError;

const SPECIAL_CHARACTERS: &str = "() \n\t";

pub struct Scanner<'input> {
    pub index: usize,
    pub input: &'input str,
}

impl<'input> Scanner<'input> {
    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.index)
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn next_char(&mut self) -> Option<char> {
        let c = self.peek();
        self.advance();
        c
    }

    pub fn scan(self) -> Result<Vec<Lexeme>, ScanError> {
        self.into_iter()
            .map(|r| r.map(Builtins::filter))
            .filter(|lexeme| ! matches!(lexeme, &Ok(Whitespace)))
            .collect()
    }

    pub fn number(&mut self, c: char) -> Option<Result<Lexeme, ScanError>> {
        let mut string = String::from(c);

        while let Some(c @ '0'..='9') = self.peek() {
            self.advance();
            string.push(c);
        }

        string.parse::<i32>()
            .map(|number| Token(Data(Int(number))))
            .map_err(ScanError::from)
            .into()
    }

    pub fn identifier(&mut self, c: char) -> Option<Result<Lexeme, ScanError>> {
        let mut string = String::from(c);

        while let Some(c) = self.peek() {
            if SPECIAL_CHARACTERS.contains(c) {
                return Some(Ok(Token(Abstract(Ident(string)))));
            }
            self.advance();
            string.push(c);
        }

        Some(Ok(Token(Abstract(Ident(string)))))
    }
}

impl<'input> Iterator for Scanner<'input> {
    type Item = Result<Lexeme, ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
        let lexeme: Self::Item =
            match self.next_char()? {
                '(' => Ok(Start),
                ')' => Ok(End),
                n @ '0'..='9' => self.number(n)?,
                ' ' | '\n' | '\t' => Ok(Whitespace),
                o => self.identifier(o)?,
            }.into();

        lexeme.into()
    }
}