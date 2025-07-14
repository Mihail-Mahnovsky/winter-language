use crate::expression_node::*;

#[derive(Debug, Clone)]
pub struct binOpNode{
    left : expressionNode,
    right : expressionNode,
    op : char,
}

impl binOpNode{
    pub fn new(left : expressionNode, right : expressionNode, op : char) -> Self{
        Self{
            left,
            right,
            op
        }
    }

    pub fn get_left(&self) -> expressionNode{
        return self.left.clone();
    }

    pub fn get_right(&self) -> expressionNode{
        return self.right.clone();
    }

    pub fn get_op(&self) -> char{
        return self.op.clone();
    }
}