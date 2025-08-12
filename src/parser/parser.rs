use core::panic;
use std::any::Any;
use std::collections::HashMap;
use std::thread::current;
use std::thread::panicking;
use std::vec;

use crate::assignmentNode;
use crate::binOpNode;
use crate::functionNode;
use crate::interpritator::objects::Object;
use crate::lexer;
use crate::parser::nodes::callNode;
use crate::parser::nodes::{expressionNode, numberNode};
use crate::returnNode;
use crate::scopeNode;
use crate::variableNode;

use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Char,
    Long,
    Short,
    Void,
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub arg_type: Type,
}

#[derive(Debug, Clone)]
pub enum Node {
    Assignment(assignmentNode),
    ExpressionNode(expressionNode),
    FunctionNode(functionNode),
    ReturnNode(returnNode),
}

pub struct Parser {
    tokens_clone: Vec<Token>,
    pos: usize,
    variable_types: HashMap<String, Type>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens_clone: Vec::new(),
            pos: 0,
            variable_types: HashMap::new(),
        }
    }

    fn current(&self) -> &Token {
        &self.tokens_clone[self.pos]
    }

    fn eat(&mut self, want_type: TokenType) {
        if self.pos < self.tokens_clone.len() && self.tokens_clone[self.pos].get_type() == want_type
        {
            self.pos += 1;
        } else {
            panic!(
                "Unexpected XYETA:  {:?}, got {:?}",
                want_type,
                self.current().get_type()
            );
        }
    }

    fn statement_list(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();

        while self.pos < self.tokens_clone.len() {
            if self.current().get_type() == TokenType::RBracket {
                break;
            }
            nodes.push(self.statement());
        }

        nodes
    }

    fn statement(&mut self) -> Node {
        let tok = self.current().clone();
        match tok.get_type() {
            TokenType::ID => {
                if self.tokens_clone.get(self.pos + 1).map(|t| t.get_type())
                    == Some(TokenType::Colon)
                {
                    self.declaration()
                } else if self.tokens_clone.get(self.pos + 1).map(|t| t.get_type())
                    == Some(TokenType::Assignment)
                {
                    self.reassignment()
                } else if self.tokens_clone.get(self.pos + 1).map(|t| t.get_type())
                    == Some(TokenType::LParen)
                {
                    let name = self.current().get_value();
                    self.eat(TokenType::ID);
                    self.eat(TokenType::LParen);
                    let mut args: Vec<expressionNode> = Vec::new();
                    while self.current().get_type() != TokenType::RParen {
                        args.push(self.expr());
                        if self.current().get_type() == TokenType::Coma {
                            self.eat(TokenType::Coma);
                        } else {
                            break;
                        }
                    }
                    self.eat(TokenType::RParen);
                    Node::ExpressionNode(
                        (expressionNode::FunctionCall((callNode::new(name, args)))),
                    )
                } else {
                    panic!("Invalid statement starting with ID");
                }
            }
            TokenType::Fn => self.custom_func(),
            TokenType::Return => {
                self.eat(TokenType::Return);
                let value = self.expr();
                Node::ReturnNode((returnNode::new(value)))
            }
            _ => Node::ExpressionNode(self.expr()),
        }
    }

    fn reassignment(&mut self) -> Node {
        let name = self.current().get_value();
        self.eat(TokenType::ID);
        self.eat(TokenType::Assignment);

        let expr = self.expr();

        let t = self
            .variable_types
            .get(&name)
            .unwrap_or_else(|| panic!("no type found for variable '{}'", name))
            .clone();

        let var_node = variableNode::new(name, t);
        Node::Assignment(assignmentNode::new(var_node, expr))
    }

    fn declaration(&mut self) -> Node {
        let name = self.current().get_value();
        self.eat(TokenType::ID);
        self.eat(TokenType::Colon);

        let t = match self.current().get_type() {
            TokenType::IntType => Type::Int,
            TokenType::StringType => Type::String,
            TokenType::BoolType => Type::Bool,
            TokenType::CharType => Type::Char,
            TokenType::LongType => Type::Long,
            TokenType::ShortType => Type::Short,
            TokenType::FloatType => Type::Float,
            _ => panic!("Unexpected type"),
        };

        self.variable_types.insert(name.clone(), t.clone());
        self.eat(self.current().get_type());

        if self.current().get_type() == TokenType::Assignment {
            self.eat(TokenType::Assignment);
            let expr = self.expr();

            let extype = match &expr {
                expressionNode::Number(_) => t == Type::Int,
                expressionNode::StringLiteral(_) => t == Type::String,
                expressionNode::DefaultValue(_) => true,
                expressionNode::Variable(var) => var.get_type() == t,
                
                expressionNode::BinOp(_) => {
                    fn is_binop_type_correct(expr: &expressionNode, expected: &Type) -> bool {
                        match expr {
                            expressionNode::BinOp(op) => {
                                is_binop_type_correct(&op.get_left(), expected)
                                    && is_binop_type_correct(&op.get_right(), expected)
                            }
                            expressionNode::Number(_) => expected == &Type::Int,
                            expressionNode::StringLiteral(_) => expected == &Type::String,
                            expressionNode::Bool(_) => expected == &Type::Bool,
                            expressionNode::CharLiteral(_) => expected == &Type::Char,
                            expressionNode::FloatExpression(_) => expected == &Type::Float,
                            expressionNode::LongExpression(_) => expected == &Type::Long,
                            expressionNode::Variable(var) => &var.get_type() == expected,
                            _ => false,
                        }
                    }

                    if !is_binop_type_correct(&expr, &t) {
                        panic!("Type mismatch in binop for variable '{}'", name);
                    }
                    true
                }

                _ => false,
            };

            //if !extype {
            //panic!("Type not need {:?}, got {:?}", t, expr);
            //}

            return Node::Assignment(assignmentNode::new(variableNode::new(name, t), expr));
        }

        Node::Assignment(assignmentNode::new(
            variableNode::new(name, t.clone()),
            expressionNode::DefaultValue(t),
        ))
    }

    fn parse_scope(&mut self) -> scopeNode {
        self.eat(TokenType::LBracket);
        let mut nodes: Vec<Node> = Vec::new();
        nodes = self.statement_list();
        self.eat(TokenType::RBracket);
        scopeNode::new(nodes)
    }

    fn custom_func(&mut self) -> Node {
        self.eat(TokenType::Fn);

        //обявления имя функции
        let name = if self.current().get_type() == TokenType::ID {
            let val = self.current().get_value();
            self.eat(TokenType::ID);
            val
        } else {
            panic!("Expected name fn got: {}", self.current().get_value());
        };

        self.eat(TokenType::LParen);

        //парс аргументов
        let mut args: Vec<Arg> = Vec::new();
        while self.current().get_type() != TokenType::RParen {
            if self.current().get_type() == TokenType::ID {
                let arg_name = self.current().get_value();
                self.eat(TokenType::ID);
                self.eat(TokenType::Colon);

                let arg_type = match self.current().get_type() {
                    TokenType::IntType => {
                        self.eat(TokenType::IntType);
                        Type::Int
                    }
                    TokenType::StringType => {
                        self.eat(TokenType::StringType);
                        Type::String
                    }
                    TokenType::BoolType => {
                        self.eat(TokenType::BoolType);
                        Type::Bool
                    }
                    TokenType::VoidType => {
                        self.eat(TokenType::VoidType);
                        Type::Void
                    }
                    _ => panic!("unknown arg type"),
                };

                args.push(Arg {
                    name: arg_name,
                    arg_type,
                });

                if self.current().get_type() == TokenType::Coma {
                    self.eat(TokenType::Coma);
                } else {
                    break;
                }
            } else {
                panic!("Expected argument identifier");
            }
        }

        self.eat(TokenType::RParen);
        for arg in &args {
            //println!("arg : {}",arg.name);
            self.variable_types
                .insert(arg.name.clone(), arg.arg_type.clone());
        }

        self.eat(TokenType::RetOp);

        //обработка возращаемого значения
        let return_val = match self.current().get_type() {
            TokenType::IntType => {
                self.eat(TokenType::IntType);
                Type::Int
            }
            TokenType::StringType => {
                self.eat(TokenType::StringType);
                Type::String
            }
            TokenType::BoolType => {
                self.eat(TokenType::BoolType);
                Type::Bool
            }
            TokenType::VoidType => {
                self.eat(TokenType::VoidType);
                Type::Void
            }
            _ => panic!("Unknown return type: {}", self.current().get_value()),
        };

        self.eat(TokenType::LBracket);

        let body_nodes = self.statement_list();

        self.eat(TokenType::RBracket);

        let scope = scopeNode::new(body_nodes);

        let func = Node::FunctionNode(functionNode::new(name, args, scope, return_val));

        func
    }

    fn expr(&mut self) -> expressionNode {
        let left = self.term();
        self.term_tail(left)
    }

    fn term_tail(&mut self, mut left: expressionNode) -> expressionNode {
        while self.pos < self.tokens_clone.len() {
            let tok = self.current().clone();
            if tok.get_type() == TokenType::Operator
                && (tok.get_value() == "+" || tok.get_value() == "-")
            {
                let op = tok.get_value().chars().next().unwrap();
                self.eat(TokenType::Operator);
                let right = self.term();
                let binop = binOpNode::new(left, right, op);
                left = expressionNode::BinOp(Box::new(binop));
            } else {
                break;
            }
        }
        left
    }

    fn term(&mut self) -> expressionNode {
        let left = self.factor();
        self.factor_tail(left)
    }

    fn factor_tail(&mut self, mut left: expressionNode) -> expressionNode {
        while self.pos < self.tokens_clone.len() {
            let tok = self.current().clone();
            if tok.get_type() == TokenType::Operator
                && (tok.get_value() == "*" || tok.get_value() == "/")
            {
                let op = tok.get_value().chars().next().unwrap();
                self.eat(TokenType::Operator);
                let right = self.factor();
                let binop = binOpNode::new(left, right, op);
                left = expressionNode::BinOp(Box::new(binop));
            } else {
                break;
            }
        }
        left
    }

    fn factor(&mut self) -> expressionNode {
        let tok = self.current().clone();
        match tok.get_type() {
            TokenType::IntLiteral => {
                let value_str = tok.get_value();
                let value: i128 = value_str.parse().expect("not number");

                self.eat(TokenType::IntLiteral);

                if value > 3600 {
                    expressionNode::LongExpression(value)
                } else {
                    expressionNode::Number(numberNode::new(value as i32))
                }
            }
            TokenType::FloatLiteral => {
                //not work
                //let value = self.current().get_value();
                //let float_value = value.parse::<f32>().unwrap();
                expressionNode::FloatExpression((0.0))
            }
            TokenType::StringLiteral => {
                self.eat(TokenType::StringLiteral);
                expressionNode::StringLiteral(tok.get_value())
            }
            TokenType::CharLiteral => {
                self.eat(TokenType::CharLiteral);
                let val = tok.get_value();
                let ch = val.chars().next().expect("Empty char literal");
                expressionNode::CharLiteral(ch)
            }
            TokenType::True => {
                self.eat(TokenType::True);
                expressionNode::Bool((true))
            }
            TokenType::False => {
                self.eat(TokenType::False);
                expressionNode::Bool((false))
            }
            TokenType::ID => {
                let name = self.current().get_value();

                if self.tokens_clone.get(self.pos + 1).map(|t| t.get_type())
                    == Some(TokenType::LParen)
                {
                    self.eat(TokenType::ID);
                    self.eat(TokenType::LParen);

                    let mut args: Vec<expressionNode> = Vec::new();

                    if self.current().get_type() != TokenType::RParen {
                        loop {
                            args.push(self.expr());
                            if self.current().get_type() == TokenType::Coma {
                                self.eat(TokenType::Coma);
                            } else {
                                break;
                            }
                        }
                    }

                    self.eat(TokenType::RParen);

                    expressionNode::FunctionCall(callNode::new(name, args))
                } else {
                    self.eat(TokenType::ID);

                    let var_type = self
                        .variable_types
                        .get(&name)
                        .unwrap_or_else(|| panic!("var '{}' not obivlena", name))
                        .clone();

                    expressionNode::Variable(variableNode::new(name, var_type))
                }
            }
            TokenType::LParen => {
                self.eat(TokenType::LParen);
                let expr = self.expr();
                self.eat(TokenType::RParen);
                expr
            }
            _ => panic!("Unexpected token in factor(xyeta): {:?}", tok),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Vec<Node> {
        self.tokens_clone = tokens;
        self.pos = 0;
        self.statement_list()
    }
}
