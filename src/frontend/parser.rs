use crate::frontend::lexer::Lexeme;
use crate::frontend::lexer::Lexeme::{CloseParen, EOF, Minus, OpenParen, Plus, Slash, Star};
use crate::ast::ast::ASTNode;
use crate::ast::ast::ASTNode::{Add, Divide, Multiply, Subtract};

/// Given an ordered collection of lexemes
/// Build an abstract syntax tree
pub fn parse(lexemes: Vec<Lexeme>) -> Option<ASTNode> {
    // There should be at least an EOF lexeme
    assert!(lexemes.len() > 0);
    if lexemes[0] == EOF {
        None
    } else {
        Some(Parser { index: 0, lexemes }.parse())
    }
}

// Contain relevant data for parsing
struct Parser {
    index: usize,
    lexemes: Vec<Lexeme>,
}

// Parse methods
impl Parser {
    fn parse(&mut self) -> ASTNode {
        let ast = self.parse_add();
        // Complain if error.
        if self.index != self.lexemes.len() - 1 {
            panic!("Expected EOF, got {:?}.\n ", self.lexemes[self.index]);
        }
        ast
    }

    fn parse_add(&mut self) -> ASTNode {
        let mut left = self.parse_multiply();

        while self.in_bounds() {
            left = match self.current() {
                Plus => {
                    self.advance();
                    Add { left: Box::new(left), right: Box::new(self.parse_multiply()) }
                }
                Minus => {
                    self.advance();
                    Subtract { left: Box::new(left), right: Box::new(self.parse_multiply()) }
                }
                _ => { return left }
            };
        }

        left
    }

    fn parse_multiply(&mut self) -> ASTNode {
        let mut left = self.parse_atom();

        while self.in_bounds() {
            left = match self.current() {
                Star => {
                    self.advance();
                    Multiply { left: Box::new(left), right: Box::new(self.parse_atom()) }
                }
                Slash => {
                    self.advance();
                    Divide { left: Box::new(left), right: Box::new(self.parse_atom()) }
                }
                _ => { return left }
            }
        }

        left
    }

    // parse atom:
    // either a parenthesized expression (EXPR)
    // Or a simple number
    fn parse_atom(&mut self) -> ASTNode {
        match self.current() {
            OpenParen => {
                self.advance();
                // Note: calling root parse WILL fail due to bounds checks.
                let value = self.parse_add();
                if !self.has(CloseParen) {
                    panic!("Unterminated parentheses!")
                }
                self.advance();
                value
            }
            _ => {
                self.parse_number()
            }
        }
    }

    fn parse_number(&mut self) -> ASTNode {
        match self.next() {
            Lexeme::Number { value } => {
                ASTNode::Number { value }
            }
            _ => { panic!("Expected a number, but none was found!"); }
        }
    }
}

// Parser helpers
impl Parser {
    fn in_bounds(&self) -> bool {
        self.index < self.lexemes.len()
    }

    // Return whether the cursor has an element of the specified type
    fn has(&self, l: Lexeme) -> bool {
        self.in_bounds() && self.lexemes[self.index] == l
    }

    // Advance to the next character.
    // Return the character that was previously under the cursor
    fn next(&mut self) -> Lexeme {
        assert!(self.in_bounds());
        let ret_val = self.current();
        self.index += 1;
        ret_val
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    // Get the current lexeme
    fn current(&self) -> Lexeme {
        assert!(self.in_bounds());
        self.lexemes[self.index]
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::ASTNode::{Add, Divide, Multiply, Number};
    use crate::frontend::lexer::lex;
    use crate::frontend::parser::parse;

    #[test]
    fn test_parse() {
        let input = "(3 + 5) * 3 / -2";
        let lexemes = lex(input);

        let three = Number { value: 3.0 };
        let five = Number { value: 5.0 };
        let plus = Add { left: Box::new(three), right: Box::new(five) };
        let three = Number { value: 3.0 };
        let times = Multiply { left: Box::new(plus), right: Box::new(three) };
        let neg_two = Number { value: -2.0 };
        let divide = Divide { left: Box::new(times), right: Box::new(neg_two) };

        assert_eq!(divide, parse(lexemes).unwrap());
    }

    #[test]
    fn test_empty() {
        let input = "";
        assert_eq!(None, parse(lex(input)))
    }

    #[test]
    #[should_panic]
    fn test_invalid_lexeme() {
        let input = "3+";
        parse(lex(input));
    }
}