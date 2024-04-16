use std::collections::HashMap;
use std::sync::OnceLock;
use crate::default_types::*;
use crate::scanner::Lexeme;

#[derive(Debug)]
pub struct Builtins(HashMap<Lexeme, Token>);

static BUILTINS: OnceLock<Builtins> = OnceLock::new();

impl Builtins {
    pub fn init() {
        BUILTINS.set(Builtins::default()).expect("Global builtins object already set!");
    }
    
    pub fn global() -> &'static Self {
        BUILTINS.get().expect("Global builtins object was not initialized!")
    }
    
    pub fn filter(arg: Lexeme) -> Lexeme {
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