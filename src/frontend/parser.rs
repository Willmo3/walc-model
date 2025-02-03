use crate::frontend::lexer::Lexeme;
use crate::frontend::lexer::Lexeme::{CloseParen, EOF, Minus, OpenParen, Plus, Slash, Star};
use crate::ast::ast::ASTNode;
use crate::ast::ast::ASTNode::{Add, Divide, Multiply, Subtract};

/// Given an ordered collection of lexemes
/// Build an abstract syntax tree
pub fn parse(lexemes: Vec<Lexeme>) -> Option<Result<ASTNode, String>> {
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
    fn parse(&mut self) -> Result<ASTNode, String> {
        let ast = self.parse_add();
        match ast {
            Ok(ast) => {
                // Complain if some of AST ignored.
                if self.index != self.lexemes.len() - 1 {
                    Err(format!("Expected EOF, got {:?}.\n ", self.lexemes[self.index]))
                } else {
                    Ok(ast)
                }
            }
            Err(error) => Err(error),
        }
    }

    fn parse_add(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_multiply();

        while self.in_bounds() && !left.is_err()
            && (self.current() == Plus || self.current() == Minus) {

            let operation = self.current();
            self.advance();

            let right = match self.parse_multiply() {
                Ok(ast) => { ast }
                Err( error ) => { return Err(error); }
            };

            match operation {
                Plus => {
                    left = Ok (Add { left: Box::new(left?), right: Box::new(right) })
                }
                Minus => {
                    left = Ok (Subtract { left: Box::new(left?), right: Box::new(right) })
                }
                _=> panic!("Internal error -- verified operation was plus or minus!")
            }
        }

        left
    }

    fn parse_multiply(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_atom();

        // End parsing subtree when error found on left.
        while self.in_bounds() && !left.is_err()
            && (self.current() == Star || self.current() == Slash) {

            let operation = self.current();
            self.advance();

            // Immediately error if problem in right subtree.
            let right = match self.parse_atom() {
                Ok(ast) => { ast }
                Err(error ) => { return Err(error) }
            };

            match operation {
                Star => {
                    left = Ok ( Multiply { left: Box::new(left?), right: Box::new(right) } )
                }
                Slash => {
                    left = Ok ( Divide { left: Box::new(left?), right: Box::new(right) } )
                }
                _ => panic!("Internal error -- verified earlier it was plus or minus!" )
            }
        }

        left
    }

    // parse atom:
    // either a parenthesized expression (EXPR)
    // Or a simple number
    fn parse_atom(&mut self) -> Result<ASTNode, String> {
        match self.current() {
            OpenParen => {
                self.advance();
                // Note: calling root parse WILL fail due to bounds checks.
                let value = self.parse_add();
                if !self.has(CloseParen) {
                    Err("Unterminated parentheses!".to_string())
                } else {
                    self.advance();
                    value
                }
            }
            _ => {
                self.parse_number()
            }
        }
    }

    fn parse_number(&mut self) -> Result<ASTNode, String> {
        match self.next() {
            Lexeme::Number { value } => {
                Ok(ASTNode::Number { value })
            }
            _ => { Err("Expected a number, but none was found!".to_string()) }
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

        assert_eq!(Ok(divide), parse(lexemes.unwrap()).unwrap());
    }

    #[test]
    fn test_empty() {
        let input = "";
        assert_eq!(None, parse(lex(input).unwrap()));
    }

    #[test]
    fn test_invalid_lexeme() {
        let input = "3+";
        assert_eq!(Some(Err("Expected a number, but none was found!".to_string())), parse(lex(input).unwrap()));
    }
}