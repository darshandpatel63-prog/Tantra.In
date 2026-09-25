pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod token;

use diagnostic::Diagnostic;
use lexer::Lexer;
use parser::{Parser, Program};

pub fn parse_source(source: &str) -> (Option<Program>, Vec<Diagnostic>) {
    let (tokens, mut diagnostics) = Lexer::new(source).lex();
    let (program, parse_diagnostics) = Parser::new(tokens).parse();
    diagnostics.extend(parse_diagnostics);
    (program, diagnostics)
}
