use crate::parser::nodes::expressionNode;

#[derive(Debug, Clone)]
pub struct echoNode {
    pub value: expressionNode,
}

impl echoNode {
    pub fn new(value: expressionNode) -> Self {
        Self { value }
    }
}
