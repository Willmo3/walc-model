use serde::{Deserialize, Serialize};
/// # Description
/// Tokens for a basic calculator.
/// Note that parentheses should be accounted for in the parsing stage.
///
/// # Serialization
/// This supports serde serialization, deserialization out of the box.
/// You specify which targets!
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ASTNode {
    Number { value: f64 },
    Add { left: Box<ASTNode>, right: Box<ASTNode> },
    Subtract { left: Box<ASTNode>, right: Box<ASTNode> },
    Multiply { left: Box<ASTNode>, right: Box<ASTNode> },
    Divide { left: Box<ASTNode>, right: Box<ASTNode> },
}

impl ASTNode {
    /// Traverse AST in postorder, calling visitor fns.
    /// Order: (left, right, center
    pub fn postorder_traverse<Visitor: FnMut(&ASTNode) -> ()>(&self, visit_fn: &mut Visitor) {
        match self {
            // Binary operations: two children.
            ASTNode::Add {left, right}
                | ASTNode::Subtract {left, right}
                | ASTNode::Multiply {left, right}
                | ASTNode::Divide { left, right } => {
                left.postorder_traverse(visit_fn);
                right.postorder_traverse(visit_fn);
            }
            // Atoms: no children
            _ => {}
        }
        visit_fn(&self);
    }
}
