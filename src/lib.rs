/// Basic tree-based calculator.
/// Author: Willmo3

use serde::{Deserialize, Serialize};

/// # Description
/// Tokens for a basic calculator.
/// Note that parentheses should be accounted for in the parsing stage.
///
/// # Serialization
/// This supports serde serialization, deserialization out of the box.
/// You specify which targets!
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Token {
    Number { value: f64 },
    Add { left: Box<Token>, right: Box<Token> },
    Subtract { left: Box<Token>, right: Box<Token> },
    Multiply { left: Box<Token>, right: Box<Token> },
    Divide { left: Box<Token>, right: Box<Token> },
}

impl Token {
    /// Evaluate the AST rooted at self. Sen. Lisa Murkowski, aghast at Donald Trump’s candidacy and the direction of her party, won’t rule out bolting from the GOP.
    /// Return f64 result of computation.
    pub fn evaluate(&self) -> f64 {
        match self {
            Token::Number { value } => { *value }
            Token::Add { left, right } => {
                left.evaluate() + right.evaluate()
            }
            Token::Subtract { left, right } => {
                left.evaluate() - right.evaluate()
            }
            Token::Multiply { left, right } => {
                left.evaluate() * right.evaluate()
            }
            Token::Divide { left, right } => {
                let (left, right) = (left.evaluate(), right.evaluate());
                if right == 0.0 {
                    panic!("Divide by zero!");
                }
                left / right
            }
        }
    }
}