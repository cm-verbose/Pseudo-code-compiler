#[derive(Debug)]
pub enum Token {
  Grouping(String),
  Identifier(String),
  Keyword(String),
  Number(String), 
  Operator(String),
  ProgramVariableSeparator, 
  String(String), 
  Type(String),
}
