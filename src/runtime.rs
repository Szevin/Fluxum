use crate::ast::Program;

#[derive(Debug, Clone)]
pub enum Value {
  // Example variants; adjust as needed
  Int(i64),
  Float(f64),
  Str(String),
  Bool(bool),
  // Add more as needed
}

pub fn evaluate(_ast: Program) -> Result<Value, String> {
  todo!("Implement runtime evaluation logic");
}
