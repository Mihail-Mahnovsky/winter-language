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
use crate::parser::parser::Type;

pub struct Interpritator{
    stack : Vec<Object>,
}

impl Interpritator{

    pub fn new() -> Self{
        Self {     
            stack: Vec::new()
        }
    }

    fn mulStr(&self, l : String, r :i32) -> String{
        let mut res = String::new();
        for i in 0..r{
            res = format!("{}{}",res,l);
        }

        return res;
    }

    fn type_matches(&mut self, var_type: &Type, val: &Object) -> bool {
    match (var_type, val) {
        (Type::Int, Object::Int(_)) => true,
        (Type::Float, Object::Float(_)) => true,
        (Type::String, Object::String(_)) => true,
        (Type::Bool, Object::Bool(_)) => true,
        (Type::Void, Object::Void) => true,
        _ => false,
        }
    }


    pub fn execute(&mut self, node : Node) {
        match node {
            Node::EchoNode(echo) => {
                let value = self.eval_expr(echo.value);
                println!("{}", value);
            },
            Node::Assignment(assign) => {
                let value = self.eval_expr(assign.get_expression());
                let index = assign.get_variable().get_index();
                if index < self.stack.len() {
                if self.type_matches(&assign.get_variable().get_type(), &value) {
                    self.stack[index] = value;
                } else {
                    panic!("Type mismatch: expected {:?}, got {:?}", assign.get_variable().get_type(), value);
                }
                    
                } else {
                    self.stack.resize(index + 1, Object::Void);
                    self.stack[index] = value;
                }
            }
            Node::ExpressionNode(expr) => {
                let _ = self.eval_expr(expr);
            },
            Node::FunctionNode(func) => {
                panic!("not function included")
            }
        }
    }

    fn eval_expr(&mut self, expr: expressionNode) -> Object {
        match expr {
            expressionNode::DefaultValue(t) => {
                match t {
                Type::Int => Object::Int(0),
                Type::String => Object::String(String::new()),
                Type::Bool => Object::Bool(false),
                Type::Float => Object::Float(0.0),
                Type::Void => Object::Void,
                }
            }
            expressionNode::Number(num_node) => {
                Object::Int((num_node.get_value()))
            }

            expressionNode::StringLiteral(str_node) => {
                Object::String((str_node))
            }
            expressionNode::Variable(var_node) => {
                let index = var_node.get_index();  
                self.stack.get(index).cloned().unwrap_or_else(|| {
                    panic!("var for index : {} exixts", index)
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
                        _ => panic!("nelzia plus, delet i misuc stroki!"),
                    },
                    (Object::String(l), Object::Int(r)) => match op {
                        '*' => Object::String(self.mulStr(l, r)),
                        _ => panic!("nelzia delet i misuc stroki"),
                    },
                    _ => panic!("ia ne znayu che zdeci napicati"),
                }
            }
        }
    }
}