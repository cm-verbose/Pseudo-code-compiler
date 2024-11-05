mod components;
use components::{parser::Parser, tokenizer::Tokenizer};

use crate::language_features::Token;
use std::{fs, path::Path};

pub struct PseudoCodeCompiler {
  code: String,
}

/**
 * A pseudo-code compiler
 */
impl PseudoCodeCompiler {
  pub fn ini() -> Self {
    PseudoCodeCompiler {
      code: String::new(),
    }
  }

  /* Compile source code from file */
  pub fn compile_from_path(&mut self, path: &Path) -> () {
    if !path.exists() {
      let err: String = if let Some(path_str) = path.to_str() {
        format!("Specified file at path \"{}\" is invalid", path_str)
      } else {
        // (handling invalid unicode)
        "Specified file does not exist".to_owned()
      };
      println!("{}", err);
      return;
    }

    if let Ok(code) = fs::read_to_string(path) {
      self.compile_from_source(code);
    } else {
      println!("File does not contain valid UTF-8");
    }
  }

  /* Compile source code from source */
  pub fn compile_from_source(&mut self, code: String) -> () {
    if code.trim().len() == 0 {
      println!("Specified source is empty");
      return;
    }
    self.compile(code);
  }

  /* Go through the compilation steps */
  fn compile(&mut self, code: String) {

    /* Needed for peeking */
    self.code = code.clone();
    let mut tokenizer: Tokenizer = Tokenizer::ini();
    let tokens: &Vec<Token> = tokenizer.tokenize(&code); 

    let mut parser: Parser = Parser::ini();
    parser.parse(tokens);
  }
}
