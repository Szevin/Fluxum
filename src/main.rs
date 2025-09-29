use std::env;
use std::fs;

use crate::parser::Parser;

mod ast;
mod lexer;
mod parser;
mod pipeline;
mod queue;
mod runtime;
mod stdlib;
mod type_checker;

fn main() {
  // -----------------------------
  // 1. Parse CLI arguments
  // -----------------------------
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: {} <script.flx>", args[0]);
    std::process::exit(1);
  }
  let script_path = &args[1];

  // -----------------------------
  // 2. Read the source code
  // -----------------------------
  let source = match fs::read_to_string(script_path) {
    Ok(s) => s,
    Err(e) => {
      eprintln!("Error reading file {}: {}", script_path, e);
      std::process::exit(1);
    }
  };

  // -----------------------------
  // 3. Lexing
  // -----------------------------
  let tokens = match lexer::tokenize(&source) {
    Ok(t) => t,
    Err(e) => {
      eprintln!("Lexing error: {}", e);
      std::process::exit(1);
    }
  };

  // -----------------------------
  // 4. Parsing
  // -----------------------------
  let mut parser = Parser::new(tokens);
  let ast = match parser.parse_program() {
    Ok(a) => a,
    Err(e) => {
      eprintln!("Parsing error: {}", e);
      std::process::exit(1);
    }
  };

  // -----------------------------
  // 5. Type Checking
  // -----------------------------
  if let Err(e) = type_checker::check(&ast) {
    eprintln!("Type checking error: {}", e);
    std::process::exit(1);
  }

  // -----------------------------
  // 6. Runtime / Evaluation
  // -----------------------------
  let result = runtime::evaluate(ast);

  // -----------------------------
  // 7. Flush sequential side-effects
  // -----------------------------
  queue::flush();

  // -----------------------------
  // 8. Handle final output
  // -----------------------------
  println!("Program result: {:?}", result);
}
