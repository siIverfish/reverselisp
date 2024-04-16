#![allow(dead_code)]

use lisp::run;
use lisp::error::{Error, ScanError};

static INPUT: &str = "1 + 1";

fn main() -> Result<(), Error> { repl() }

fn program() -> Result<(), Error> {
    let result = run(INPUT)?;
    dbg!(&result);
    Ok(())
}

fn repl() -> Result<(), Error> {
    loop { repl_once()? }
}

fn repl_once() -> Result<(), Error> {
    use std::io::stdin;
    let mut input = String::new();
    stdin().read_line(&mut input).map_err(ScanError::from)?;
    let input = input.trim();
    
    let result = run(input)?;
    dbg!(&result);
    Ok(())
}
