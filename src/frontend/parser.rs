use std::str::FromStr;
use crate::frontend::lexer::Lexeme;
use crate::ast::ast::ASTNode;
use crate::ast::ast::ASTNode::{Add, Divide, Multiply, Subtract};
use crate::frontend::lexer::LexemeType::{CloseParen, DoubleStar, Minus, Numeric, OpenParen, Plus, Slash, Star, EOF};

/// Given an ordered collection of lexemes
/// Build an abstract syntax tree
pub fn parse(lexemes: Vec<Lexeme>) -> Option<Result<ASTNode, String>> {
    // There should be at least an EOF lexeme
    assert!(lexemes.len() > 0);
    if lexemes[0].lexeme_type == EOF {
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

        // If left is an error message, prime our error reporting with its data
        let mut err_message = if let Err(message) = &left {
            message.clone()
        } else {
            String::new()
        };

        // Even if already errored, we will continue attempting to parse to gain more errors.
        while self.in_bounds()
            && (self.current().lexeme_type == Plus || self.current().lexeme_type == Minus) {

            let operation = self.current().lexeme_type;
            self.advance();

            let right = match self.parse_multiply() {
                Ok( ast) => { Ok(ast) }
                Err( error ) => { err_message.push_str(&error); Err(error) }
            };

            if !left.is_err() && !right.is_err() {
                match operation {
                    Plus => {
                        left = Ok (Add { left: Box::new(left?), right: Box::new(right?) })
                    }
                    Minus => {
                        left = Ok (Subtract { left: Box::new(left?), right: Box::new(right?) })
                    }
                    _=> panic!("Internal error -- verified operation was plus or minus!")
                }
            }
        }

        if !err_message.is_empty() {
            Err(err_message)
        } else {
            Ok(left?)
        }
    }

    fn parse_multiply(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_exponentiation();

        // If left is an error message, prime our error reporting with its data
        let mut err_message = if let Err(message) = &left {
            message.clone()
        } else {
            String::new()
        };

        // Even if error found, will attempt to continue parsing to gain more errors
        while self.in_bounds()
            && (self.current().lexeme_type == Star || self.current().lexeme_type == Slash) {

            let operation = self.current().lexeme_type;
            self.advance();

            // Immediately error if problem in right subtree.
            let right = match self.parse_atom() {
                Ok(ast) => { Ok(ast) }
                Err(error ) => { err_message.push_str(&error); Err(error) }
            };

            if !left.is_err() && !right.is_err() {
                match operation {
                    Star => {
                        left = Ok ( Multiply { left: Box::new(left?), right: Box::new(right?) } )
                    }
                    Slash => {
                        left = Ok ( Divide { left: Box::new(left?), right: Box::new(right?) } )
                    }
                    _ => panic!("Internal error -- verified earlier it was plus or minus!" )
                }
            }
        }

        if !err_message.is_empty() {
            Err(err_message)
        } else {
            Ok(left?)
        }
    }

    fn parse_exponentiation(&mut self) -> Result<ASTNode, String> {
        // Root of right associative exponentiation ast.
        let mut root_expression = self.parse_atom();

        let mut err_message = if let Err(message) = &root_expression {
            message.clone()
        } else {
            String::new()
        };

        // If the next lexeme in the stream is a double star (exponentiation), recurse!
        // Right associativity makes recursive implementation more efficient.
        if self.in_bounds() && self.current().lexeme_type == DoubleStar {
            // Skip doublestar literal.
            self.advance();

            // Recurse on right subtree to implement right associativity.
            let right = match self.parse_exponentiation() {
                Ok(ast) => { Ok(ast) }
                Err(error ) => { err_message.push_str(&error); Err(error) }
            };

            // Return error if message empty.
            if !root_expression.is_err() && !right.is_err() {
                root_expression = Ok(ASTNode::Exponentiate { left: Box::new(root_expression?), right: Box::new(right?) })
            } else {
                root_expression = Err(err_message)
            }
        }
        // Base case: no doublestar on horizon.
        // Since all Walc expressions must end with a number, descend here.
        root_expression
    }

    // parse atom:
    // either a parenthesized expression (EXPR)
    // Or a simple number
    fn parse_atom(&mut self) -> Result<ASTNode, String> {
        match self.current().lexeme_type {
            OpenParen => {
                self.advance();
                // Note: calling root parse WILL fail due to bounds checks.
                let value = self.parse_add();
                if !(self.current().lexeme_type == CloseParen) {
                    Err(format!("Expected ')' on line {}, got {} instead.\n", self.current().line, self.current().text ))
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
        // Only consume input if a valid number found!
        match self.current().lexeme_type {
            Numeric => {
                let value = Ok(ASTNode::Number { value: f64::from_str(&self.current().text).unwrap() });
                self.advance();
                value
            }
            _ => Err(format!("Expected number on line {}, got {} instead.\n",
                            self.current().line, self.current().text))
        }
    }
}

// Parser helpers
impl Parser {
    fn in_bounds(&self) -> bool {
        self.index < self.lexemes.len()
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn current(&self) -> &Lexeme {
        assert!(self.in_bounds());
        &self.lexemes[self.index]
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::ASTNode::{Add, Divide, Exponentiate, Multiply, Number};
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
        assert_eq!(Some(Err("Expected number on line 1, got end of file instead.\n".to_string())), parse(lex(input).unwrap()));
    }

    #[test]
    fn test_multiple_errors() {
        let input = "3 * +";
        assert_eq!(Some(Err("Expected number on line 1, got + instead.\nExpected number on line 1, got end of file instead.\n".to_string())), parse(lex(input).unwrap()));
    }

    #[test]
    fn test_triple_exponentiation() {
        let input = "3 ** 2 ** 1";

        let three = Number { value: 3.0 };
        let two = Number { value: 2.0 };
        let one  = Number { value: 1.0 };

        let right_exp = Exponentiate { left: Box::new(two), right: Box::new(one) };
        let left_exp = Exponentiate { left: Box::new(three), right: Box::new(right_exp) };

        assert_eq!(left_exp, parse(lex(input).unwrap()).unwrap().unwrap());
    }
}