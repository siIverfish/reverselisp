use std::collections::HashMap;
use std::sync::OnceLock;
use crate::scanner::Lexeme;
use crate::{Token, Token::*, AbstractItem::*, FunctionItem::*};

#[derive(Debug)]
pub struct Builtins(HashMap<Lexeme, Token>);

static BUILTINS: OnceLock<Builtins> = OnceLock::new();

impl Builtins {
    pub fn global() -> &'static Self {
        BUILTINS.get_or_init(Builtins::default)
    }
    
    pub fn from_lexeme(arg: Lexeme) -> Lexeme {
        Self::global().0
            .get(&arg)
            .cloned()
            .map(Lexeme::Token)
            .unwrap_or(arg)
    }
}

impl Default for Builtins {
    fn default() -> Self {
        Builtins(HashMap::from([
            (Lexeme::Token(Abstract(Ident("+".into()))), Function(Adder)),
        ]))
    }
}
