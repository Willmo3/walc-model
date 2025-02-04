use std::str::FromStr;
use crate::frontend::lexer::Lexeme::{CloseParen, EOF, Minus, Number, OpenParen, Plus, Slash, Star};

/// Given a string "data" containing the source code.
/// Return a list of lexemes associated with that source
pub fn lex(data: &str) -> Result<Vec<Lexeme>, String> {
    let chars = data.chars().collect();
    let mut lexer = Lexer { data: chars, index: 0, lexemes: vec![], errors: String::new() };
    lexer.lex();
    // Attempt to lex entire program before reporting errors.
    if lexer.errors.is_empty() {
        Ok(lexer.lexemes)
    } else {
        Err(lexer.errors)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lexeme {
    Number { value: f64 }, // Coerce all numbers to floats
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Star,
    Slash,
    // Special token that all files are terminated by
    EOF,
}

struct Lexer {
    data: Vec<char>,
    index: usize,
    lexemes: Vec<Lexeme>,
    errors: String,
}

// Lexing framework
impl Lexer {
    // Lex all tokens in self's data
    fn lex(&mut self) {
        let mut lexeme_result = self.lex_next();

        while lexeme_result != Ok(EOF) {
            match &lexeme_result {
                Ok(lexeme) => { self.lexemes.push(*lexeme); },
                Err(message) => { self.errors.push_str(message); }
            }
            lexeme_result = self.lex_next()
        }
        // Push the final EOF lexeme.
        self.lexemes.push(EOF);
    }

    // lex the next token
    // Return that token to be added.
    fn lex_next(&mut self) -> Result<Lexeme, String> {
        // At the start of each token parsing, skip all whitespaces.
        while self.in_bounds() && self.current().is_whitespace() {
            self.skip()
        }
        // If after skipping whitespaces, out of bounds, return EOF .
        if !self.in_bounds() {
            return Ok(EOF)
        }
        // Otherwise, another non-whitespace character remains to be lexed.
        let start = self.next();
        match start {
            '(' => { Ok(OpenParen) }
            ')' => { Ok(CloseParen) }
            '*' => { Ok(Star) }
            '/' => { Ok(Slash) }
            '+' => { Ok(Plus) }
            // trouble: minus can be the start of a negative number.
            '-' => {
                if self.in_bounds() && self.current().is_ascii_digit() {
                    self.lex_number(start)
                } else {
                    Ok(Minus)
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
            return Ok(Number { value: f64::from_str(&chars).unwrap() });
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

        Ok( Number { value: f64::from_str(&chars).unwrap() })
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
    use crate::frontend::lexer::lex;
    use crate::frontend::lexer::Lexeme::{CloseParen, EOF, Number, OpenParen, Plus, Slash, Star};

    #[test]
    fn test_lex() {
        let input = "(3 + 5) * 3 / -2";
        let expected = vec![OpenParen, Number { value: 3.0 },
                            Plus, Number { value: 5.0 }, CloseParen, Star, Number { value: 3.0 },
                            Slash, Number { value: -2.0 }, EOF];
        let tokens = lex(input);
        assert_eq!(Ok(expected), tokens);
    }

    #[test]
    fn test_empty() {
        let input = "";
        assert_eq!(Ok(vec![EOF]), lex(input));
    }

    #[test]
    fn test_multiple_errors() {
        let input = "3. + 5.";
        assert_eq!(Err("Unterminated float.\nUnterminated float.\n".to_string()), lex(input));
    }
}