use crate::language_features::{SyntaxTree, Token, TokenType};

pub struct Parser {
  tree: SyntaxTree,
  ptr: usize,
}

impl Parser {
  pub fn ini() -> Self {
    Parser {
      tree: SyntaxTree { nodes: Vec::new() },
      ptr: 0,
    }
  }

  /* Construct an abstract syntax tree from the provided tokens */
  pub fn parse(&mut self, tokens: &Vec<Token>) -> &SyntaxTree {
    while let Some(token) = tokens.get(self.ptr) {
      match token.token_type {
        TokenType::COLON => {

        }
        _ => {}
      }
      self.ptr += 1;
    }
    return &self.tree; 
  }
}
