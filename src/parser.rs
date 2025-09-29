use crate::ast::*;
use crate::lexer::Token; // the AST types we just defined

pub struct Parser {
  tokens: Vec<Token>,
  pos: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, pos: 0 }
  }

  // main entrypoint
  pub fn parse_program(&mut self) -> Result<Program, String> {
    let mut body = Vec::new();
    while !self.is_at_end() {
      body.push(self.parse_stmt()?);
    }
    Ok(Program { body })
  }

  // ------------------------
  // Statements
  // ------------------------
  fn parse_stmt(&mut self) -> Result<Stmt, String> {
    // peek next token, decide which parse_* to call
    todo!()
  }

  fn parse_let(&mut self) -> Result<Stmt, String> {
    let mutable = self.expect(&Token::Mut).is_ok();
    let name = match self.advance() {
      Some(Token::Name(n)) => n,
      Some(token) => return Err(format!("Expected variable name, found {:?}", token)),
      None => return Err("Expected variable name, found end of input".to_string()),
    };
    let ty = match self.expect(&Token::Colon) {
      Ok(_) => Some(self.parse_type_expr()?),
      Err(_) => None,
    };
    let value = match self.expect(&Token::Assign) {
      Ok(_) => Some(self.parse_expr()?),
      Err(_) => {
        if !mutable {
          return Err("Immutable variable must have an initial value".to_string());
        } else {
          None
        }
      }
    };

    Ok(Stmt::Let {
      mutable,
      name,
      ty,
      value,
    })
  }
  fn parse_const(&mut self) -> Result<Stmt, String> {
    let name = match self.advance() {
      Some(Token::Name(n)) => n,
      Some(token) => return Err(format!("Expected constant name, found {:?}", token)),
      None => return Err("Expected constant name, found end of input".to_string()),
    };

    let ty = match self.expect(&Token::Colon) {
      Ok(_) => self.parse_type_expr()?,
      Err(_) => return Err("Constant must have a type annotation".to_string()),
    };

    let value = match self.expect(&Token::Assign) {
      Ok(_) => self.parse_expr()?,
      Err(_) => return Err("Constant must have an initial value".to_string()),
    };

    Ok(Stmt::Const { name, ty, value })
  }
  fn parse_fn(&mut self) -> Result<Stmt, String> {
    todo!()
  }
  fn parse_return(&mut self) -> Result<Stmt, String> {
    todo!()
  }
  fn parse_if(&mut self) -> Result<Stmt, String> {
    todo!()
  }
  fn parse_loop(&mut self) -> Result<Stmt, String> {
    todo!()
  }
  fn parse_match(&mut self) -> Result<Stmt, String> {
    todo!()
  }
  fn parse_type(&mut self) -> Result<Stmt, String> {
    todo!()
  }
  fn parse_enum(&mut self) -> Result<Stmt, String> {
    todo!()
  }

  // ------------------------
  // Expressions
  // ------------------------
  fn parse_expr(&mut self) -> Result<Expr, String> {
    self.parse_binary_expr(0) // start with lowest precedence
  }

  fn parse_type_expr(&mut self) -> Result<TypeExpr, String> {
    todo!()
  }

  fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
    todo!()
  }
  fn parse_unary_expr(&mut self) -> Result<Expr, String> {
    todo!()
  }
  fn parse_primary(&mut self) -> Result<Expr, String> {
    todo!()
  }
  fn parse_call(&mut self, callee: Expr) -> Result<Expr, String> {
    todo!()
  }
  fn parse_block(&mut self) -> Result<Expr, String> {
    todo!()
  }

  // ------------------------
  // Helpers
  // ------------------------
  fn peek(&self) -> Option<&Token> {
    self.tokens.get(self.pos)
  }
  fn advance(&mut self) -> Option<Token> {
    let next_token = self.tokens.get(self.pos).cloned();
    self.pos += 1;

    next_token
  }
  fn is_at_end(&self) -> bool {
    self.pos >= self.tokens.len()
  }
  fn expect(&mut self, expected: &Token) -> Result<(), String> {
    if let Some(token) = self.peek() {
      if token == expected {
        self.advance();
        Ok(())
      } else {
        Err(format!("Expected {:?}, found {:?}", expected, token))
      }
    } else {
      Err(format!("Expected {:?}, found end of input", expected))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ast::{BinaryOp, Expr, Pattern, Stmt, TypeExpr, UnaryOp};
  use crate::lexer::tokenize;
  use crate::parser::Parser;

  #[test]
  fn test_parse_let_mut_with_type_and_value() {
    let src = "let mut x: i64 = 42";
    let tokens = tokenize(src).unwrap();
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_stmt().unwrap();
    assert_eq!(
      stmt,
      Stmt::Let {
        mutable: true,
        name: "x".to_string(),
        ty: Some(TypeExpr::Named("i64".to_string())),
        value: Some(Expr::Number(42)),
      }
    );
  }

  #[test]
  fn test_parse_let_immutable_with_type_and_value() {
    let src = "let x: i32 = 10";
    let tokens = tokenize(src).unwrap();
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_stmt().unwrap();
    assert_eq!(
      stmt,
      Stmt::Let {
        mutable: false,
        name: "x".to_string(),
        ty: Some(TypeExpr::Named("i32".to_string())),
        value: Some(Expr::Number(10)),
      }
    );
  }

  #[test]
  fn test_parse_let_mut_without_type_with_value() {
    let src = "let mut y = 5";
    let tokens = tokenize(src).unwrap();
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_stmt().unwrap();
    assert_eq!(
      stmt,
      Stmt::Let {
        mutable: true,
        name: "y".to_string(),
        ty: None,
        value: Some(Expr::Number(5)),
      }
    );
  }

  #[test]
  fn test_parse_let_mut_with_type_without_value() {
    let src = "let mut z: bool";
    let tokens = tokenize(src).unwrap();
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_stmt().unwrap();
    assert_eq!(
      stmt,
      Stmt::Let {
        mutable: true,
        name: "z".to_string(),
        ty: Some(TypeExpr::Named("bool".to_string())),
        value: None,
      }
    );
  }

  #[test]
  fn test_parse_let_immutable_without_value_should_error() {
    let src = "let a: i64";
    let tokens = tokenize(src).unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_stmt();
    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      "Immutable variable must have an initial value".to_string()
    );
  }

  #[test]
  fn test_parse_let_immutable_without_type_with_value() {
    let src = "let b = 99";
    let tokens = tokenize(src).unwrap();
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_stmt().unwrap();
    assert_eq!(
      stmt,
      Stmt::Let {
        mutable: false,
        name: "b".to_string(),
        ty: None,
        value: Some(Expr::Number(99)),
      }
    );
  }
}
