use crate::number_node::numberNode;
use crate::variable_node::variableNode;
use crate::bin_op_node::binOpNode;

#[derive(Debug, Clone)]
pub enum expressionNode {
    Number(numberNode),
    Variable(variableNode),
    BinOp(Box<binOpNode>),
    StringLiteral(String),
}