use crate::number_node::numberNode;
use crate::variable_node::variableNode;
use crate::bin_op_node::binOpNode;
use crate::parser::parser::Type;
use crate::callNode;

#[derive(Debug, Clone)]
pub enum expressionNode {
    Number(numberNode),
    Variable(variableNode),
    BinOp(Box<binOpNode>),
    StringLiteral(String),
    DefaultValue(Type),
    FunctionCall(callNode),
}