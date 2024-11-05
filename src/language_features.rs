#![allow(non_camel_case_types)]

/*

Lexer

*/

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
  COMMA,
  COLON,
  EQUALS,
  LEFT_PARENTHESIS,
  RIGHT_PARENTHESIS,

  // Operators
  OP_PLUS,
  OP_MINUS,
  OP_STAR,
  OP_DIVIDE,

  OP_ASSIGN,
  OP_LESS_THAN_OR_EQUAL,
  OP_LESS_THAN,
  OP_GREATER_THAN_OR_EQUAL,
  OP_GREATER_THAN,

  // Keywords
  KW_AND,
  KW_CASE,
  KW_DO,
  KW_DEFAULT,
  KW_ELSE,
  KW_END,
  KW_FOR,
  KW_IF,
  KW_NOT,
  KW_OR,
  KW_READ,
  KW_TYPE,
  KW_WHILE,
  KW_WRITE,

  // Types
  TY_INT,
  TY_REAL,
  TY_STRING,
  TY_BOOL,

  // Type declaration sections
  DEF_TY_VARIABLES,
  DEF_TY_CONSTANTS,

  IDENTIFIER,
  NUMBER,
  STRING,

  //
  PROGRAM_SEPARATOR,
}

#[derive(Debug)]
pub struct Token {
  pub content: String,
  pub token_type: TokenType,
}

/*

Parser

*/

#[derive(Debug)]
pub enum OperationType {
  SYMBOL_DECLARATION,
}

#[derive(Debug)]
pub struct TreeNode {
  pub operation_type: OperationType,
  pub children: Vec<TreeNode>,
}

#[derive(Debug)]
pub struct SyntaxTree {
  pub nodes: Vec<TreeNode>,
}
