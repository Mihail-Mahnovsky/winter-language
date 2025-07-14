use std::collections::HashMap;

use crate::interpritator::objects::*;
use crate::assignmentNode;
use crate::binOpNode;
use crate::echoNode;
use crate::expression_node;
use crate::parser::nodes::expressionNode;
use crate::parser::parser::Node;
use crate::variableNode;
use crate::numberNode;
use crate::parser::nodes;

pub struct Interpritator{
    variables: HashMap<String, Object>,
}

impl Interpritator{

    pub fn new() -> Self{
        Self {     
            variables: HashMap::new() 
        }
    }

    fn mulStr(&self, l : String, r :i32) -> String{
        let mut res = String::new();
        for i in 0..r{
            res = format!("{}{}",res,l);
        }

        return res;
    }

    pub fn execute(&mut self, node : Node) {
        match node {
            Node::EchoNode(echo) => {
                let value = self.eval_expr(echo.value);
                println!("{}", value);
            },
            Node::Assignment(assign) => {
                let var_name = assign.get_variable().get_name();
                let value = self.eval_expr(assign.get_expression());
                self.variables.insert(var_name, value);
            }
            Node::ExpressionNode(expr) => {
                let _ = self.eval_expr(expr);
            }
        }
    }

    fn eval_expr(&mut self, expr: expressionNode) -> Object {
        match expr {
            expressionNode::Number(num_node) => {
                Object::Int((num_node.get_value()))
            }

            expressionNode::StringLiteral(str_node) => {
                Object::String((str_node))
            }
            expressionNode::Variable(var_node) => {
                let name = var_node.get_name();
                self.variables.get(&name).cloned().unwrap_or_else(|| {
                    panic!("variable : '{}' not have reasization", name)
                })
            }
            expressionNode::BinOp(boxed_op) => {
                let left = self.eval_expr(boxed_op.get_left());
                let right = self.eval_expr(boxed_op.get_right());
                let op = boxed_op.get_op();

                match (left, right) {
                    (Object::Int(l), Object::Int(r)) => match op {
                        '+' => Object::Int(l + r),
                        '-' => Object::Int(l - r),
                        '*' => Object::Int(l * r),
                        '/' => { if r == 0 { panic!("ne deli na 0!"); } Object::Int(l / r) }
                        _ => panic!("don`t support operator : '{}'", op),
                    },
                    (Object::String(l), Object::String(r)) => match op {
                        '+' => Object::String(l + &r),
                        _ => panic!("eblan nelzia delet i misuc stroki!"),
                    },
                    (Object::String(l), Object::Int(r)) => match op {
                        '*' => Object::String(self.mulStr(l, r)),
                        _ => panic!("eblan nelzia delet i misuc stroki!"),
                    },
                    _ => panic!("ia ne znayu che zdeci napicati"),
                }
            }
        }
    }
}