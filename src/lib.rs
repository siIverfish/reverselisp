#![feature(box_patterns)]
#![feature(let_chains)]

structstruck::strike! {
    #[strikethrough[derive(Debug, Hash, PartialEq, Eq, Clone)]]
    pub enum Token {
        /// There are three types of values:
        /// Data, which represents a conventional, concrete datum like 4.0 or "hi!".
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

// python would kill me for these imports
mod default_types { 
    pub use super::*;
    pub use {Token::*, FunctionItem::*, DataItem::*, AbstractItem::*}; 
    pub use EvalError::*;
}

use default_types::*;

use thiserror::Error;

#[derive(Error, Debug)]
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