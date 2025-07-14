use crate::variable_node::variableNode;
use crate::expression_node::expressionNode;

#[derive(Debug, Clone)]
pub struct assignmentNode{
    left : variableNode,
    right : expressionNode,
}

impl assignmentNode{
    pub fn new(left : variableNode, right : expressionNode) -> Self{
        Self{
            left,
            right,
        }
    }

    pub fn get_variable(&self) -> variableNode{
        return self.left.clone();
    }

    pub fn get_expression(&self) -> expressionNode{
        return self.right.clone();
    }
}