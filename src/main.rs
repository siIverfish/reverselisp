use lisp::scanner::Scanner;
use lisp::parser::parse;

type Error = Result<(), Box<dyn std::error::Error>>;

static INPUT: &str = "1 + 1";
 
fn main() -> Error { repl() }

fn program() -> Error {
    println!("main() running...");
    
    lisp::builtins::Builtins::init();
    run(INPUT)
}

fn run(input: &str) -> Error {
    let scanner = Scanner { index: 0, input };

    let lexemes = scanner.scan()?;
    // dbg!(&lexemes);

    let tree = parse(&mut lexemes.into_iter())?;
    dbg!(&tree);

    let result = tree.get()?;
    dbg!(result);
    
    Ok(())
}

fn repl() -> Error {
    lisp::builtins::Builtins::init();
    loop { repl_once()? }
}

fn repl_once() -> Error {
    use std::io::stdin;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input = input.trim();
    
    run(input)
}
