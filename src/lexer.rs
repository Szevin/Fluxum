#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  // Keywords
  Let,
  Mut,
  Fn,
  Ret,
  Loop,
  Skip,
  Stop,
  If,
  Match,
  Type,
  Enum,
  Pub,
  Const,
  Async,
  Await,
  Yield,

  // Symbols
  ParenthesisOpen,
  ParenthesisClose,
  BraceOpen,
  BraceClose,
  BracketOpen,
  BracketClose,
  Comma,
  Colon,

  // Operators
  Plus,
  Minus,
  Asterisk,
  Slash,
  Percent,
  Equal,
  NotEqual,
  GreaterThan,
  LessThan,
  GreaterEqual,
  LessEqual,
  And,
  Or,
  Not,
  Pipe,
  Assign,

  // Literals and Identifiers
  Number(i64),
  String(String),
  Boolean(bool),
  Name(String),

  // Comments
  LineComment,
  BlockCommentOpen,
  BlockCommentClose,

  Illegal(usize, usize),
}

impl Token {
  fn from_str(s: &str, line: usize, column: usize) -> Token {
    match s {
      "let" => Token::Let,
      "mut" => Token::Mut,
      "fn" => Token::Fn,
      "return" => Token::Ret,
      "loop" => Token::Loop,
      "skip" => Token::Skip,
      "stop" => Token::Stop,
      "if" => Token::If,
      "match" => Token::Match,
      "type" => Token::Type,
      "enum" => Token::Enum,
      "pub" => Token::Pub,
      "const" => Token::Const,
      "async" => Token::Async,
      "await" => Token::Await,
      "yield" => Token::Yield,

      "(" => Token::ParenthesisOpen,
      ")" => Token::ParenthesisClose,
      "{" => Token::BraceOpen,
      "}" => Token::BraceClose,
      "[" => Token::BracketOpen,
      "]" => Token::BracketClose,
      "," => Token::Comma,
      ":" => Token::Colon,

      "+" => Token::Plus,
      "-" => Token::Minus,
      "*" => Token::Asterisk,
      "/" => Token::Slash,
      "%" => Token::Percent,
      "==" => Token::Equal,
      "!=" => Token::NotEqual,
      ">" => Token::GreaterThan,
      "<" => Token::LessThan,
      ">=" => Token::GreaterEqual,
      "<=" => Token::LessEqual,
      "&&" => Token::And,
      "||" => Token::Or,
      "!" => Token::Not,
      "|>" => Token::Pipe,
      "=" => Token::Assign,

      "//" => Token::LineComment,
      "/*" => Token::BlockCommentOpen,
      "*/" => Token::BlockCommentClose,

      "true" => Token::Boolean(true),
      "false" => Token::Boolean(false),

      _ if s.chars().all(|c| c.is_ascii_digit()) => match s.parse::<i64>() {
        Ok(num) => Token::Number(num),
        Err(_) => Token::Illegal(line, column),
      },
      _ if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 => {
        Token::String(s[1..s.len() - 1].to_string())
      }
      _ if s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !s.chars().next().unwrap().is_ascii_digit() =>
      {
        Token::Name(s.to_string())
      }

      _ => Token::Illegal(line, column),
    }
  }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
  let mut line_number = 1;
  let mut column_number = 1;
  let mut tokens = Vec::new();

  let lines = source.lines();

  for line in lines {
    for word in line.split_whitespace() {
      tokens.push(Token::from_str(word, line_number, column_number));

      column_number += word.len() + 1; // +1 for the space
    }
    column_number = 1;
    line_number += 1;
  }

  Ok(tokens)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tokenize_keywords() {
    let src = "let mut fn return loop skip stop if match type enum pub const async await yield";
    let result = tokenize(src).unwrap();
    let expected_tokens = vec![
      Token::Let,
      Token::Mut,
      Token::Fn,
      Token::Ret,
      Token::Loop,
      Token::Skip,
      Token::Stop,
      Token::If,
      Token::Match,
      Token::Type,
      Token::Enum,
      Token::Pub,
      Token::Const,
      Token::Async,
      Token::Await,
      Token::Yield,
    ];
    for (i, token) in result.iter().enumerate() {
      assert_eq!(token, &expected_tokens[i]);
    }
    assert_eq!(result.len(), expected_tokens.len());
  }

  #[test]
  fn test_tokenize_symbols() {
    let src = "( ) { } [ ] ,";
    let result = tokenize(src).unwrap();
    let expected_tokens = vec![
      Token::ParenthesisOpen,
      Token::ParenthesisClose,
      Token::BraceOpen,
      Token::BraceClose,
      Token::BracketOpen,
      Token::BracketClose,
      Token::Comma,
    ];
    for (i, token) in result.iter().enumerate() {
      assert_eq!(token, &expected_tokens[i]);
    }
    assert_eq!(result.len(), expected_tokens.len());
  }

  #[test]
  fn test_tokenize_operators() {
    let src = "+ - * / % = == != > < >= <= && || !
  |>";
    let result = tokenize(src).unwrap();
    let expected_tokens = vec![
      Token::Plus,
      Token::Minus,
      Token::Asterisk,
      Token::Slash,
      Token::Percent,
      Token::Assign,
      Token::Equal,
      Token::NotEqual,
      Token::GreaterThan,
      Token::LessThan,
      Token::GreaterEqual,
      Token::LessEqual,
      Token::And,
      Token::Or,
      Token::Not,
      Token::Pipe,
    ];
    for (i, token) in result.iter().enumerate() {
      assert_eq!(token, &expected_tokens[i]);
    }
    assert_eq!(result.len(), expected_tokens.len());
  }

  #[test]
  fn test_tokenize_literals_and_identifiers() {
    let src = r#"42 "hello" true false my_var anotherVar123"#;
    let result = tokenize(src).unwrap();
    let expected_tokens = vec![
      Token::Number(42),
      Token::String("hello".to_string()),
      Token::Boolean(true),
      Token::Boolean(false),
      Token::Name("my_var".to_string()),
      Token::Name("anotherVar123".to_string()),
    ];
    for (i, token) in result.iter().enumerate() {
      assert_eq!(token, &expected_tokens[i]);
    }
    assert_eq!(result.len(), expected_tokens.len());
  }

  #[test]
  fn test_tokenize_comments() {
    let src = "// /* */";
    let result = tokenize(src).unwrap();
    let expected_tokens = vec![
      Token::LineComment,
      Token::BlockCommentOpen,
      Token::BlockCommentClose,
    ];
    for (i, token) in result.iter().enumerate() {
      assert_eq!(token, &expected_tokens[i]);
    }
    assert_eq!(result.len(), expected_tokens.len());
  }

  #[test]
  fn test_tokenize_illegal() {
    let src = "@ # $ ^ & ~ `";
    let result = tokenize(src).unwrap();
    let expected_tokens = vec![
      Token::Illegal(1, 1),
      Token::Illegal(1, 3),
      Token::Illegal(1, 5),
      Token::Illegal(1, 7),
      Token::Illegal(1, 9),
      Token::Illegal(1, 11),
      Token::Illegal(1, 13),
    ];
    for (i, token) in result.iter().enumerate() {
      assert_eq!(token, &expected_tokens[i]);
    }
    assert_eq!(result.len(), expected_tokens.len());
  }
}
