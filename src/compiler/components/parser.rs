use crate::language_features::{SyntaxTree, Token};

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
  pub fn parse(&mut self, tokens: &Vec<Token>) {
    println!("{:?}", self.tree); 
    while let Some(token) = tokens.get(self.ptr) {
      println!("{:?}", token); 
      self.ptr += 1;
    }
  }
}
