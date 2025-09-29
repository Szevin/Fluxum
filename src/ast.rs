pub struct Program {
  pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
  // variable stuff
  Let {
    mutable: bool,
    name: String,
    ty: Option<TypeExpr>,
    value: Option<Expr>,
  },
  Const {
    name: String,
    ty: TypeExpr,
    value: Expr,
  },

  // functions
  Fn {
    name: String,
    params: Vec<Param>,
    ret_type: Option<TypeExpr>,
    body: Vec<Stmt>,
    is_async: bool,
    is_pub: bool,
  },

  Return(Option<Expr>),
  Yield(Expr),
  Await(Expr),

  // control flow
  If {
    condition: Expr,
    then_branch: Vec<Stmt>,
    else_branch: Option<Vec<Stmt>>,
  },
  Loop(Vec<Stmt>),
  Skip,
  Stop,

  // types & enums
  Type {
    name: String,
    definition: TypeExpr,
    is_pub: bool,
  },
  Enum {
    name: String,
    variants: Vec<EnumVariant>,
    is_pub: bool,
  },

  // generic expression
  ExprStmt(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Number(i64),
  String(String),
  Boolean(bool),
  Identifier(String),

  // operators
  Binary {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
  },
  Unary {
    op: UnaryOp,
    expr: Box<Expr>,
  },

  // calls & access
  Call {
    callee: Box<Expr>,
    args: Vec<Expr>,
  },
  Field {
    base: Box<Expr>,
    field: String,
  },

  // match
  Match {
    value: Box<Expr>,
    arms: Vec<(Pattern, Vec<Stmt>)>,
  },

  // block as expression
  Block(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Eq,
  Neq,
  Gt,
  Lt,
  Ge,
  Le,
  And,
  Or,
  Pipe,
  Assign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
  Neg,
  Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpr {
  Named(String), // e.g. "i32"
  Generic {
    base: String, // e.g. "Option"
    args: Vec<TypeExpr>,
  },
  Function {
    params: Vec<TypeExpr>,
    ret: Box<TypeExpr>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
  Literal(Expr),
  Identifier(String),
  Wildcard, // _
  EnumVariant {
    enum_name: String,
    variant: String,
    fields: Vec<Pattern>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
  pub name: String,
  pub ty: TypeExpr,
  pub is_mut: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
  pub name: String,
  pub fields: Vec<TypeExpr>, // tuple-style variants
}
