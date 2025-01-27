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
    /// Traverse AST in postorder, calling visitor fns.
    /// Order: (left, right, center
    pub fn postorder_traverse<Visitor: FnMut(&Token) -> ()>(&self, visit_fn: &mut Visitor) {
        match self {
            // Binary operations: two children.
            Token::Add {left, right}
                | Token::Subtract {left, right}
                | Token::Multiply {left, right}
                | Token::Divide { left, right } => {
                left.postorder_traverse(visit_fn);
                right.postorder_traverse(visit_fn);
            }
            // Atoms: no children
            _ => {}
        }
        visit_fn(&self);
    }
}
