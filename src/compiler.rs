use crate::language_features::{Token, TokenType};
use std::{collections::HashMap, fs, path::Path};

pub struct PseudoCodeCompiler {
  code: String,
  ptr: usize,
  line: u128,
  tokens: Vec<Token>,
  keywords: HashMap<TokenType, String>,
}

/**
 * A pseudo-code compiler.
 */
impl PseudoCodeCompiler {
  pub fn ini() -> Self {
    PseudoCodeCompiler {
      code: String::new(),
      tokens: Vec::new(),
      line: 1u128,
      ptr: 0,
      keywords: HashMap::new(),
    }
  }

  /* Create the keywords */
  pub fn generate_keywords(&self) -> HashMap<TokenType, String> {
    let mut keyword_maps: HashMap<TokenType, String> = HashMap::new();

    /* Defining our keywords here */
    keyword_maps.insert(TokenType::KW_AND, "ET".to_owned());
    keyword_maps.insert(TokenType::KW_CASE, "CAS".to_owned());
    keyword_maps.insert(TokenType::KW_DEFAULT, "AUTRE".to_owned());
    keyword_maps.insert(TokenType::KW_DO, "FAIRE".to_owned());
    keyword_maps.insert(TokenType::KW_ELSE, "SINON".to_owned());
    keyword_maps.insert(TokenType::KW_END, "FIN".to_owned());
    keyword_maps.insert(TokenType::KW_FOR, "POUR".to_owned());
    keyword_maps.insert(TokenType::KW_IF, "SI".to_owned());
    keyword_maps.insert(TokenType::KW_OR, "OU".to_owned());
    keyword_maps.insert(TokenType::KW_NOT, "NON".to_owned());
    keyword_maps.insert(TokenType::KW_READ, "LIRE".to_owned());
    keyword_maps.insert(TokenType::KW_TYPE, "TYPE".to_owned());
    keyword_maps.insert(TokenType::KW_WHILE, "TANTQUE".to_owned());
    keyword_maps.insert(TokenType::KW_WRITE, "ECRIRE".to_owned());

    /* Defining types here */
    keyword_maps.insert(TokenType::TY_BOOL, "bool".to_owned());
    keyword_maps.insert(TokenType::TY_INT, "entier".to_owned());
    keyword_maps.insert(TokenType::TY_REAL, "réel".to_owned());
    keyword_maps.insert(TokenType::TY_STRING, "chaine".to_owned());

    return keyword_maps;
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
    self.reset();

    /* Needed for peeking */
    self.code = code.clone();

    self.tokenize(&code);
    self.parse();
  }

  /* Tokenize (or lex) the source code */
  fn tokenize(&mut self, code: &String) -> &Vec<Token> {
    let chars: Vec<char> = code.chars().collect();

    while self.ptr < code.len() {
      if let None = chars.get(self.ptr) {
        break;
      }
      let current_char: char = *chars.get(self.ptr).unwrap();
      let char_string: String = current_char.to_string();

      match current_char {
        ',' => self.add_token(TokenType::COMMA, char_string),
        ':' => self.add_token(TokenType::COLON, char_string),
        '+' => self.add_token(TokenType::OP_PLUS, char_string),
        '-' => self.add_token(TokenType::OP_MINUS, char_string),
        '*' => self.add_token(TokenType::OP_STAR, char_string),
        '/' => self.add_token(TokenType::OP_DIVIDE, char_string),
        '=' => self.add_token(TokenType::EQUALS, char_string),
        '(' => self.add_token(TokenType::LEFT_PARENTHESIS, char_string),
        ')' => self.add_token(TokenType::RIGHT_PARENTHESIS, char_string),

        /* Compound operators */
        '<' => {
          if let Some(seq) = self.peek(2) {
            if seq == "<-" {
              self.add_token(TokenType::OP_ASSIGN, seq);
              self.ptr += 2;
              continue;
            } else if seq == "<=" {
              self.add_token(TokenType::OP_LESS_THAN_OR_EQUAL, seq);
              self.ptr += 1;
              continue;
            }
          }
          self.add_token(TokenType::OP_LESS_THAN, char_string);
        }

        '>' => {
          if let Some(seq) = self.peek(2) {
            if seq == ">=" {
              self.add_token(TokenType::OP_GREATER_THAN_OR_EQUAL, seq);
              self.ptr += 2;
              continue;
            }
          }
          self.add_token(TokenType::OP_GREATER_THAN, char_string);
        }
        '_' => {
          let mut program_separator_content: String = String::new();
          if let Some(seq) = self.peek(5) {
            if seq == "_".repeat(5) {
              while {
                let current: Option<&char> = chars.get(self.ptr);
                current.is_some() && *current.unwrap() == '_'
              } {
                program_separator_content.push('_');
                self.ptr += 1;
              }
              self.add_token(TokenType::PROGRAM_SEPARATOR, program_separator_content);
            }
          }
        }
        '\n' => self.line += 1,
        _ => {}
      }

      // Identifiers
      if current_char.is_alphabetic() {
        let mut identifier_content: String = String::new();

        while {
          let current: Option<&char> = chars.get(self.ptr);
          current.is_some() && current.unwrap().is_alphabetic()
        } {
          let current: char = *chars.get(self.ptr).unwrap();
          identifier_content.push(current);
          self.ptr += 1;
        }

        if let Some(key) = self.keywords.iter().find_map(|(key, val)| {
          if *val == identifier_content {
            Some(key)
          } else {
            None
          }
        }) {
          let token_type: TokenType = key.clone();
          self.add_token(token_type, identifier_content);
        } else {
          self.add_token(TokenType::IDENTIFIER, identifier_content);
        }
        continue;
      }

      // Numbers
      if current_char.is_ascii_digit() {
        let mut number_content: String = String::new();
        let mut number_of_points: u8 = 0;

        while {
          let current: Option<&char> = chars.get(self.ptr);
          current.is_some() && (current.unwrap().is_ascii_digit() || *current.unwrap() == '.')
        } {
          let current: char = *chars.get(self.ptr).unwrap();
          if current == '.' && number_of_points < 2 {
            number_of_points += 1;
          }
          number_content.push(current);
          self.ptr += 1;
        }

        /* Numbers can't have more than two poins */
        if number_of_points >= 2 {
          let message = format!("Number \"{}\" contains more than two dots", number_content);
          self.report_error(message.as_str());
        }
        self.add_token(TokenType::NUMBER, number_content);
        continue;
      }

      // Strings
      if current_char == '\"' {
        let mut string_content: String = String::new();
        self.ptr += 1;

        while *chars.get(self.ptr).unwrap_or(&'\"') != '\"' {
          let current: char = *chars.get(self.ptr).unwrap();
          if current == '\n' {
            self.line += 1;
          }
          string_content.push(current);
          self.ptr += 1;
        }

        if self.ptr == code.len() {
          self.report_error("String is unterminated");
        }
        self.add_token(TokenType::STRING, string_content);
      }
      self.ptr += 1;
    }
    return &self.tokens;
  }

  /* Observes fowards in the string */
  fn peek(&self, peek_length: usize) -> Option<String> {
    let range_start: usize = self.ptr;
    let range_end: usize = self.ptr + peek_length;

    if range_end > self.code.len() && range_start <= self.code.len() {
      None
    } else {
      Some(self.code[range_start..range_end].to_string())
    }
  }

  /* Add a token to the list */
  fn add_token(&mut self, token_type: TokenType, token_content: String) {
    self.tokens.push(Token {
      content: token_content,
      token_type,
    })
  }

  /* Parse tokens to return an abstract syntax tree */
  pub fn parse(&mut self) {
    for token in &self.tokens {
      println!("{:?}", token);
    }
  }

  /* Report a fatal error */
  fn report_error(&self, message: &str) -> ! {
    panic!("ERROR : [Line {}] {}", self.line, message);
  }

  /* Reset the internal state of the parser for multiple compilations */
  fn reset(&mut self) {
    self.code = String::new();
    self.tokens = Vec::new();
    self.ptr = 0;
    self.line = 1u128;
    self.keywords = self.generate_keywords();
  }
}
