use std::collections::HashMap;

use crate::assignmentNode;
use crate::binOpNode;
use crate::echoNode;
use crate::expression_node;
use crate::interpritator::function::*;
use crate::interpritator::objects::*;
use crate::numberNode;
use crate::parser::nodes;
use crate::parser::nodes::expressionNode;
use crate::parser::parser::Node;
use crate::parser::parser::Type;
use crate::variableNode;

#[derive(Debug, Clone)]
pub struct Interpritator {
    variables: HashMap<String, Object>,
    functions: HashMap<String, function>,
}

impl Interpritator {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn mulStr(&self, l: String, r: i32) -> String {
        let mut res = String::new();
        for i in 0..r {
            res = format!("{}{}", res, l);
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

    pub fn execute(&mut self, node: Node) {
        match node {
            Node::EchoNode(echo) => {
                let value = self.eval_expr(echo.value);
                println!("{}", value);
            }
            Node::Assignment(assign) => {
                let value = self.eval_expr(assign.get_expression());
                let name = assign.get_variable().get_name();
                self.variables.insert(name, value);
            }
            Node::ExpressionNode(expr) => {
                let _ = self.eval_expr(expr);
            }
            Node::FunctionNode(func) => {
                let name = func.get_name();
                let args = func.get_args();
                let scope = func.get_scope();
                let ret_val = func.get_ret_val();
                self.functions
                    .insert(name.clone(), function::new(name, scope, args, ret_val));
            }
        }
    }

    fn get_clone_node(&self, c_fn: &function) -> Vec<Node> {
        let mut nodess: Vec<Node> = Vec::new();
        for i in 0..c_fn.get_scope().get_nodes_len() {
            if let Some(no) = c_fn.get_scope().get_nodes().get(i) {
                nodess.push(no.clone());
            }
        }
        nodess
    }

    fn eval_expr(&mut self, expr: expressionNode) -> Object {
        match expr {
            expressionNode::DefaultValue(t) => match t {
                Type::Int => Object::Int(0),
                Type::String => Object::String(String::new()),
                Type::Bool => Object::Bool(false),
                Type::Float => Object::Float(0.0),
                Type::Void => Object::Void,
            },
            expressionNode::Number(num_node) => Object::Int((num_node.get_value())),

            expressionNode::StringLiteral(str_node) => Object::String((str_node)),
            expressionNode::Variable(var_node) => {
                let name = var_node.get_name();
                self.variables
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Переменная '{}' не определена", name))
            }
            expressionNode::BinOp(boxed_op) => {
                let left: Object = self.eval_expr(boxed_op.get_left());
                let right = self.eval_expr(boxed_op.get_right());
                let op = boxed_op.get_op();

                match (left, right) {
                    (Object::Int(l), Object::Int(r)) => match op {
                        '+' => Object::Int(l + r),
                        '-' => Object::Int(l - r),
                        '*' => Object::Int(l * r),
                        '/' => {
                            if r == 0 {
                                panic!("ne deli na 0!");
                            }
                            Object::Int(l / r)
                        }
                        _ => panic!("don`t support operator : '{}'", op),
                    },
                    (Object::String(l), Object::String(r)) => match op {
                        '+' => Object::String(l + &r),
                        _ => panic!("nelzia mul, delet i misuc stroki!"),
                    },
                    (Object::String(l), Object::Int(r)) => match op {
                        '*' => Object::String(self.mulStr(l, r)),
                        _ => panic!("nelzia delet i misuc stroki"),
                    },
                    _ => panic!("ia ne znayu che zdeci napicati"),
                }
            }
            expressionNode::FunctionCall(call) => {
                let name = call.get_function_name();
                let args = call.get_arguments();

                if let Some(c_fn) = self.functions.get(&name) {
                    let nodes = { self.get_clone_node(c_fn) };
                    for node in nodes.iter() {
                        self.execute(node.clone());
                    }
                }

                return Object::Void;
            }
        }
    }
}
