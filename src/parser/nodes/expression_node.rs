use crate::bin_op_node::binOpNode;
use crate::callNode;
use crate::number_node::numberNode;
use crate::parser::parser::Type;
use crate::variable_node::variableNode;

#[derive(Debug, Clone)]
pub enum expressionNode {
    Number(numberNode),
    LongExpression(i128),
    FloatExpression(f32),
    Variable(variableNode),
    BinOp(Box<binOpNode>),
    StringLiteral(String),
    DefaultValue(Type),
    FunctionCall(callNode),
    CharLiteral(char),
    Bool(bool),
}
