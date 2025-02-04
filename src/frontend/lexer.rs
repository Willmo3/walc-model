use crate::frontend::lexer::LexemeType::{CloseParen, Minus, Numeric, OpenParen, Plus, Slash, Star, EOF};

/// Given a string "data" containing the source code.
/// Return a list of lexemes associated with that source
pub fn lex(data: &str) -> Result<Vec<Lexeme>, String> {
    let chars = data.chars().collect();
    let mut lexer = Lexer { data: chars, index: 0, lexemes: vec![], errors: String::new(), line: 1};
    lexer.lex();
    // Attempt to lex entire program before reporting errors.
    if lexer.errors.is_empty() {
        Ok(lexer.lexemes)
    } else {
        Err(lexer.errors)
    }
}

// Lexeme fields.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LexemeType {
    Numeric, // Coerce all numbers to floats
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Star,
    Slash ,
    // Special token that all files are terminated by
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub(crate) lexeme_type: LexemeType,
    line: usize,
    pub(crate) text: String,
}
impl Lexeme {
    pub fn new(lexeme_type: LexemeType, line: usize, text: String) -> Lexeme {
        Lexeme { lexeme_type, line, text }
    }
}

struct Lexer {
    data: Vec<char>,
    index: usize,
    lexemes: Vec<Lexeme>,
    errors: String,
    line: usize,
}

// Lexing framework
impl Lexer {
    // Lex all tokens in self's data
    fn lex(&mut self) {
        let mut lexeme_result = self.lex_next();
        loop {
            match lexeme_result {
                Ok(lexeme) => {
                    self.lexemes.push(lexeme.clone());
                    // Terminate lexing when EOF found.
                    if lexeme.lexeme_type == EOF {
                        break;
                    }
                }
                Err (message) => {
                    self.errors.push_str(&message);
                }
            }
            lexeme_result = self.lex_next();
        }
    }

    fn lex_next(&mut self) -> Result<Lexeme, String> {
        // At the start of each token parsing, skip all whitespaces.
        while self.in_bounds() && self.current().is_whitespace() {
            // Track lines in source code.
            if self.current() == '\n' {
                self.line += 1;
            }
            self.skip()
        }
        // If after skipping whitespaces, out of bounds, return EOF .
        if !self.in_bounds() {
            return Ok(Lexeme::new(EOF, self.line, String::new()));
        }
        // Otherwise, another non-whitespace character remains to be lexed.
        let start = self.next();
        match start {
            '(' => {
                Ok(Lexeme::new(OpenParen, self.line, String::from("(")))
            }
            ')' => {
                Ok(Lexeme::new(CloseParen, self.line, String::from(")")))
            }
            '*' => {
                Ok(Lexeme::new(Star, self.line, String::from("*")))
            }
            '/' => {
                Ok(Lexeme::new(Slash, self.line, String::from("/")))
            }
            '+' => {
                Ok(Lexeme::new(Plus, self.line, String::from("+")))
            }
            // trouble: minus can be the start of a negative number.
            '-' => {
                if self.in_bounds() && self.current().is_ascii_digit() {
                    self.lex_number(start)
                } else {
                    Ok( Lexeme::new (Minus, self.line, String::from("-")) )
                }
            }
            _ => {
                if start.is_ascii_digit() {
                    self.lex_number(start)
                } else {
                    Err(format!("Unexpected character: '{}'.\n", self.current()))
                }
            }
        }
    }
}

// Literal lexers
impl Lexer {
    // Lex a numeric literal, starting with character char.
    // All numbers are converted to floats.
    fn lex_number(&mut self, start: char) -> Result<Lexeme, String> {
        assert!(start.is_numeric() || start == '-');

        // Collect all the characters used to build this number.
        let mut chars = start.to_string();
        // Push all the numbers until we encounter a non-numeric character.
        while self.in_bounds() && self.current().is_ascii_digit() {
            chars.push(self.next());
        }

        // If the next character isn't a decimal point, we've got an integer.
        if !self.in_bounds() || self.current() != '.' {
            return Ok( Lexeme::new(Numeric, self.line, chars))
        }

        // Otherwise, treat it as a decimal number.
        chars.push(self.next());
        // Floats must have a value after the decimal point!
        if !self.in_bounds() || !self.current().is_ascii_digit() {
            return Err("Unterminated float.\n".to_string());
        }

        while self.in_bounds() && self.current().is_ascii_digit() {
            chars.push(self.next());
        }

        Ok( Lexeme::new(Numeric, self.line, chars ))
    }
}

// Assorted helpers
impl Lexer {
    fn in_bounds(&self) -> bool {
        self.index < self.data.len()
    }

    // Advance to the next character.
    // Return the character that was previously under the cursor
    fn next(&mut self) -> char {
        // OK to assert here -- failure indicates internal error.
        assert!(self.in_bounds());
        let ret_val = self.current();
        self.index += 1;
        ret_val
    }

    // Skip the next character
    fn skip(&mut self) {
        self.index += 1;
    }

    // Get the current character
    fn current(&self) -> char {
        assert!(self.in_bounds());
        self.data[self.index]
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::lexer::{lex, Lexeme};
    use crate::frontend::lexer::LexemeType::{Numeric, OpenParen, Plus, Slash, Star, CloseParen, EOF};

    #[test]
    fn test_lex() {
        let input = "(3 + 5)\n * 3 / -2";
        let expected = vec![
            Lexeme::new(OpenParen, 1, String::from("(")),
            Lexeme::new(Numeric, 1, String::from("3")),
            Lexeme::new(Plus, 1, String::from("+")),
            Lexeme::new(Numeric, 1, String::from("5")),
            Lexeme::new(CloseParen, 1, String::from(")")),
            Lexeme::new(Star, 2, String::from("*")),
            Lexeme::new(Numeric, 2, String::from("3")),
            Lexeme::new(Slash, 2, String::from("/")),
            Lexeme::new(Numeric, 2, String::from("-2")),
            Lexeme::new(EOF, 2, String::new())
                            ];
        let tokens = lex(input);
        assert_eq!(Ok(expected), tokens);
    }

    #[test]
    fn test_empty() {
        let input = "";
        assert_eq!(Ok(vec![Lexeme::new(EOF, 1, String::new())]), lex(input));
    }

    #[test]
    fn test_multiple_errors() {
        let input = "3. + 5.";
        assert_eq!(Err("Unterminated float.\nUnterminated float.\n".to_string()), lex(input));
    }
}