pub mod compiler;
use compiler::PseudoCodeCompiler;

pub mod language_features;
use std::path::Path;

fn main() {
  let mut pseudo_code_compiler: PseudoCodeCompiler = PseudoCodeCompiler::ini();
  let target:&Path = Path::new("./programs/program.pseudo");
  pseudo_code_compiler.compile_from_path(target);
}
