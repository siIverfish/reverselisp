#![feature(box_patterns)]
#![feature(let_chains)]

use {Token::*, FunctionItem::*, DataItem::*, AbstractItem::*};
use crate::parser::parse;
use crate::scanner::Scanner;
use crate::error::{EvalError, EvalError::*};

structstruck::strike! {
    #[strikethrough[derive(Debug, Hash, PartialEq, Eq, Clone)]]
    pub enum Token {
        /// There are three types of values:
        /// Data, which represent a conventional, concrete datum like 4 or "hi!".
        /// Function, something that acts on data.
        /// Abstract, an entity that will, when prompted, resolve itself into a value.
        Data(enum DataItem {
            Int(i32),
            // List(Box<Token>, Box<Token>),
            Args(Box<(Token, Token)>),
        }),
        Function(enum FunctionItem {
            Add(Box<Token>),
            Adder,
            Evaluate,
        }),
        Abstract(enum AbstractItem {
            Ident(String),
            Applied(Box<pub struct AppliedItem {
                f: Token,
                to: Token,
            }>)
        }),
    }
}

pub fn run(input: &str) -> Result<Token, error::Error> {
    let scanner = Scanner { index: 0, input };
    let lexemes = scanner.scan()?;
    let tree = parse(&mut lexemes.into_iter())?;
    let result = tree.get()?;
    Ok(result)
}

impl FunctionItem {
    pub fn eval(self, arg: Token) -> Result<Token, EvalError> {
        match self {
            Adder => Ok(Function(Add(Box::new(arg)))),
            Add(box x) => match (x.get()?, arg.get()?) {
                (Data(Int(a)), Data(Int(b))) => Ok(Data(Int(a + b))),
                (a, b) => Err(RuntimeError(format!("Could not add '{a:?}' to '{b:?}'")))
            }
            Evaluate => arg.get(),
        }
    }
}

impl Token {
    pub fn eval(self, arg: Token) -> Result<Token, EvalError> {
        if let Data(Args(box (first, second))) = arg {
            self.eval(first)?.eval(second)
        } else {
            match self {
                data @ Data(_) => match arg {
                    more_data @ Data(_) => Ok(Data(Args(Box::new((data, more_data))))),
                    other => other.eval(data),
                },
                Function(f) => f.eval(arg),
                Abstract(m) => m.get()?.eval(arg),
            }
        }
    }
    
    pub fn get(self) -> Result<Token, EvalError> {
        match self {
            Abstract(abstract_item) => abstract_item.get(),
            other => Ok(other),
        }
    }
}

impl AbstractItem {
    pub fn get(self) -> Result<Token, EvalError> {
        match self {
            Applied(box AppliedItem { f, to }) => f.eval(to),
            Ident(string) => Err(CouldNotResolve(string))
        }
    }
}

pub mod scanner;
pub mod error;
pub mod builtins;
pub mod parser;