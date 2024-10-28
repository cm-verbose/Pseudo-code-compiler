pub mod compiler;
use compiler::PseudoCodeCompiler;

pub mod language_features;
use language_features::Token; 
use std::{fs, path::Path};

fn main() {
  let mut pseudo_code_parser = PseudoCodeCompiler::ini();

  let target = Path::new("./programs/program.pseudo");
  if !target.exists() {
    println!("It seems that this file doesn't exist");
  }

  if let Ok(code) = fs::read_to_string(target) {
    let tokens: &Vec<Token> = pseudo_code_parser.compile(code);
    for item in tokens {
      println!("{:?}", item);
    }
  } else {
    println!("Failed opening file");
  }
}
