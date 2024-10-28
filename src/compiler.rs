use crate::language_features::Token;

pub struct PseudoCodeCompiler {
  code: String,
  ptr: usize,
  tokens: Vec<Token>,
  reserved: Vec<&'static str>,
  named_operators: Vec<&'static str>,
  types: Vec<&'static str>,
}

impl PseudoCodeCompiler {
  pub fn ini() -> Self {
    PseudoCodeCompiler {
      code: String::new(),
      ptr: 0,
      tokens: Vec::new(),
      reserved: vec![
        "AUTRE", "ALORS", "CAS", "ECRIRE", "ET", "FAIRE", "FIN", "LIRE", "NOT", "OU", "POUR", "SI",
        "SINON", "TANTQUE", "TYPE",
      ],
      named_operators: vec!["égale"],
      types: vec!["entier", "réel", "chaine", "bool"],
    }
  }

  /* Compile code to a target language (hopefully) */
  pub fn compile(&mut self, code: String) -> &Vec<Token> {
    self.reset();
    let tokens: &Vec<Token> = self.lex(code);
    return tokens;
  }

  /* Parses selected code */
  pub fn lex(&mut self, code: String) -> &Vec<Token> {
    self.code = code;
    let code_chars: Vec<char> = self.code.chars().collect();

    while self.ptr < self.code.len() {
      let cur: Option<&char> = code_chars.get(self.ptr);
      if let None = cur {
        break;
      };

      let current_char: char = *cur.unwrap();
      match current_char {
        '<' => {
          if let Some(sequence) = self.peek(2) {
            if sequence == "<-" || sequence == "<=" {
              self.ptr += 1;
              self.tokens.push(Token::Operator(sequence))
            } else if self.peek(1).unwrap() == "<" {
              self.tokens.push(Token::Operator("<".to_string()));
            }
          }
        }

        '>' => {
          if let Some(sequence) = self.peek(2) {
            let conditional_seq: String = if sequence == ">=" {
              self.ptr += 1;
              ">="
            } else {
              ">"
            }
            .to_string();
            self.tokens.push(Token::Operator(conditional_seq));
          }
        }

        /* Parentheses */
        '(' | ')' => {
          let grouping_symbol = current_char.to_string();
          self.tokens.push(Token::Grouping(grouping_symbol));
        }

        /* Operators with no composite variation */
        ',' | ':' | '=' | '+' | '-' | '*' | '/' | '%' => {
          let op_string: String = current_char.to_string();
          self.tokens.push(Token::Operator(op_string));
        }
        _ => {}
      }

      /* Indentifiers */
      if current_char.is_alphabetic() || current_char == '_' {
        let mut identifier: String = String::new();

        /* yikes! */
        while code_chars.get(self.ptr).is_some()
          && (code_chars.get(self.ptr).unwrap().is_alphanumeric()
            || *code_chars.get(self.ptr).unwrap() == '_')
        {
          identifier.push(*code_chars.get(self.ptr).unwrap());
          self.ptr += 1;
        }

        /* yikes! */
        if self.reserved.contains(&identifier.as_str()) {
          self.tokens.push(Token::Keyword(identifier));
        } else if self.named_operators.contains(&identifier.as_str()) {
          self.tokens.push(Token::Operator(identifier));
        } else if self.types.contains(&identifier.as_str()) {
          self.tokens.push(Token::Type(identifier));
        } else if identifier.chars().all(|x| x == '_') && identifier.len() >= 5 {
          self.tokens.push(Token::ProgramVariableSeparator);
        } else {
          self.tokens.push(Token::Identifier(identifier));
        }
        self.ptr -= 1;
      }

      /* Numbers */
      if current_char.is_numeric() {
        let mut number: String = current_char.to_string();
        self.ptr += 1;

        let mut point_count: u8 = 0;

        /* yikes! */
        while code_chars.get(self.ptr).is_some()
          && (code_chars.get(self.ptr).unwrap().is_ascii_digit()
            || *code_chars.get(self.ptr).unwrap() == '.')
        {
          let current = *code_chars.get(self.ptr).unwrap();
          if current == '.' && point_count < 1 {
            point_count += 1;
          }
          number.push(current);
          self.ptr += 1;
        }

        if point_count > 1 {
          panic!("{} is an invalid number, since it contains multiple dots", number);
        }
        self.tokens.push(Token::Number(number));
      }

      /* Strings */
      if current_char == '\"' {
        let mut string: String = String::new();
        self.ptr += 1;
        while code_chars.get(self.ptr).is_some() && *code_chars.get(self.ptr).unwrap() != '\"' {
          string.push(*code_chars.get(self.ptr).unwrap());
          self.ptr += 1;
        }
        self.tokens.push(Token::String(string));
      }
      self.ptr += 1;
    }

    return &self.tokens;
  }

  /* Resets the state of the compiler (for each compilation) */
  fn reset(&mut self) {
    self.code = String::new();
    self.ptr = 0;
  }

  /* Look fowards in the string (from current position) */
  fn peek(&self, peek_length: usize) -> Option<String> {
    let range_start: usize = self.ptr + 2;
    let range_end: usize = range_start + peek_length;

    if range_end >= self.code.len() && range_start <= self.code.len() {
      None
    } else {
      Some(self.code[range_start..range_end].to_string())
    }
  }
}
