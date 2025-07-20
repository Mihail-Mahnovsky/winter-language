use std::collections::HashMap;
use std::f32::consts::E;
use std::thread::scope;

use crate::assignmentNode;
use crate::binOpNode;
use crate::echoNode;
use crate::expression_node;
use crate::interpritator::function::*;
use crate::interpritator::objects::*;
use crate::numberNode;
use crate::parser::nodes;
use crate::parser::nodes::expressionNode;
use crate::parser::nodes::scopeNode;
use crate::parser::parser::Arg;
use crate::parser::parser::Node;
use crate::parser::parser::Type;
use crate::variableNode;

#[derive(Debug, Clone)]
pub enum State {
    Continue,
    Return(Object),
}

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

    pub fn execute(&mut self, node: Node) -> State {
        match node {
            Node::EchoNode(echo) => {
                let value = self.eval_expr(echo.value);
                println!("{}", value);
                State::Continue
            }
            Node::Assignment(assign) => {
                let value = self.eval_expr(assign.get_expression());
                let name = assign.get_variable().get_name();
                self.variables.insert(name, value);
                State::Continue
            }
            Node::ExpressionNode(expr) => {
                let _ = self.eval_expr(expr);
                State::Continue
            }
            Node::FunctionNode(func) => {
                let name = func.get_name();
                let args = func.get_args();
                let scope = func.get_scope();
                let ret_val = func.get_ret_val();
                self.functions
                    .insert(name.clone(), function::new(name, scope, args, ret_val));
                State::Continue
            }
            Node::ReturnNode(ret_node) => {
                let value = self.eval_expr(ret_node.get_return_value());
                State::Return((value))
            }
        }
    }

    fn get_clone_node(&self, scope: &scopeNode) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        for node in scope.get_nodes() {
            nodes.push(node.clone());
        }
        nodes
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
                    .unwrap_or_else(|| panic!("var '{}' not exits", name))
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

                let mut evaled_args = Vec::new();

                for arg in &args {
                    evaled_args.push(self.eval_expr(arg.clone()));
                }
                let temp = self.variables.clone();

                let scope_nodes = if let Some(c_fn) = self.functions.get(&name) {
                    let f_args = c_fn.get_args().clone();
                    let scope = c_fn.get_scope();

                    for (i, value) in evaled_args.iter().enumerate() {
                        match &args[i] {
                            expressionNode::StringLiteral(_) => {
                                if f_args[i].arg_type == Type::String {
                                    let name = f_args[i].name.clone();
                                    scope.add_variable(
                                        name,
                                        Object::String(value.as_string().unwrap().clone()),
                                    );
                                } else {
                                    panic!("error with types");
                                }
                            }
                            expressionNode::Number(_) => {
                                if f_args[i].arg_type == Type::Int {
                                    let name = f_args[i].name.clone();
                                    scope.add_variable(
                                        name,
                                        Object::Int(value.as_int().unwrap().clone()),
                                    );
                                } else if f_args[i].arg_type == Type::Float {
                                    let name = f_args[i].name.clone();
                                    scope.add_variable(
                                        name,
                                        Object::Float(value.as_float().unwrap().clone()),
                                    );
                                } else {
                                    panic!("error with types");
                                }
                            }
                            expressionNode::BinOp(_) => {
                                let name = f_args[i].name.clone();

                                match &value {
                                    Object::Int(v) => {
                                        if f_args[i].arg_type != Type::Int {
                                            panic!(
                                                "type mismatch: expected {:?}, got Int",
                                                f_args[i].arg_type
                                            );
                                        }
                                        scope.add_variable(name, Object::Int(*v));
                                    }

                                    Object::String(s) => {
                                        if f_args[i].arg_type != Type::String {
                                            panic!(
                                                "type mismatch: expected {:?}, got String",
                                                f_args[i].arg_type
                                            );
                                        }
                                        scope.add_variable(name, Object::String(s.clone()));
                                    }

                                    _ => panic!(
                                        "BinOp result must be Int or String, got {:?}",
                                        value
                                    ),
                                }
                            }
                            expressionNode::Variable(var) => {
                                if f_args[i].arg_type == var.get_type() {
                                    let name = f_args[i].name.clone();

                                    match var.get_type() {
                                        Type::Int => {
                                            scope.add_variable(
                                                name,
                                                Object::Int(
                                                    value.as_int().expect("need Int value"),
                                                ),
                                            );
                                        }
                                        Type::String => {
                                            scope.add_variable(
                                                name,
                                                Object::String(
                                                    value.as_string().expect("need String value"),
                                                ),
                                            );
                                        }
                                        Type::Bool => {
                                            scope.add_variable(
                                                name,
                                                Object::Bool(
                                                    value.as_bool().expect("need Bool value"),
                                                ),
                                            );
                                        }
                                        Type::Float => {
                                            scope.add_variable(
                                                name,
                                                Object::Float(
                                                    value.as_float().expect("need Float value"),
                                                ),
                                            );
                                        }
                                        Type::Void => {
                                            panic!("Cannot assign variable of type Void");
                                        }
                                    }
                                } else {
                                    panic!(
                                        "Type mismatch in function argument '{}': expected {:?}, got {:?}",
                                        f_args[i].name,
                                        f_args[i].arg_type,
                                        var.get_type()
                                    );
                                }
                            }

                            _ => panic!("govvvmo"),
                        }
                    }

                    self.variables = scope.get_variables();
                    Some(scope.get_nodes().clone())
                } else {
                    None
                };

                if let Some(scope_nodes) = scope_nodes {
                    let mut return_value = Object::Void;

                    for node in scope_nodes {
                        match self.execute(node) {
                            State::Continue => continue,
                            State::Return(value) => {
                                return_value = value;
                                break;
                            }
                        }
                    }

                    self.variables = temp;

                    if let Some(c_fn) = self.functions.get(&name) {
                        if return_value.get_type() == c_fn.get_return_value() {
                            return return_value;
                        } else {
                            panic!("func return")
                        }
                    }
                }
                return Object::Void;
            }
        }
    }
}
