use crate::converter::{convert, ConversionError};
use crate::emitter::{emit, EmitError};
use crate::eval::{eval_ast, EvalError};
use crate::parser::{format_errors, parse};
use crate::typechecker::{typecheck, TypeError};
use chumsky::error::Simple;
use std::error::Error;
use std::fs;
use thiserror::Error;

mod converter;
mod emitter;
mod eval;
mod parser;
mod typechecker;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("{}", format_errors(_1, _0))]
    ParseError(String, Vec<Simple<char>>),

    #[error(transparent)]
    TypeError(#[from] TypeError),

    #[error(transparent)]
    EmitError(#[from] EmitError),

    #[error(transparent)]
    ConversionError(#[from] ConversionError),

    #[error(transparent)]
    EvalError(#[from] EvalError),
}

fn compile(source: &str) -> Result<String, CompileError> {
    let ast = parse(source).map_err(|e| CompileError::ParseError(source.to_string(), e))?;

    let ast = convert(ast)?;

    typecheck(&ast)?;
    let final_program = eval_ast(ast)?;

    let res = emit(final_program)?;

    Ok(res)
}

fn error_main() -> Result<(), Box<dyn Error>> {
    let inp = fs::read_to_string("example.sd")?;
    let res = compile(&inp)?;
    fs::write("output.html", res)?;

    Ok(())
}

fn main() {
    if let Err(e) = error_main() {
        println!("{}", e)
    }
}
