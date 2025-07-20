use core::panic;
use std::any::Any;
use std::collections::HashMap;
use std::thread::current;
use std::thread::panicking;
use std::vec;

use crate::assignmentNode;
use crate::binOpNode;
use crate::echoNode;
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
    EchoNode(echoNode),
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
                    == Some(TokenType::COLON)
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
            TokenType::Echo => {
                self.eat(TokenType::Echo);
                let expr: expressionNode = self.expr();
                Node::EchoNode(echoNode::new(expr))
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
        self.eat(TokenType::COLON);

        let t = match self.current().get_type() {
            TokenType::IntType => Type::Int,
            TokenType::StringType => Type::String,
            TokenType::BoolType => Type::Bool,
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
                expressionNode::BinOp(op) => {
                    let left = op.get_left();
                    let right = op.get_right();

                    match (left, right) {
                        (expressionNode::Number(_), expressionNode::Number(_)) => t == Type::Int,
                        (expressionNode::StringLiteral(_), expressionNode::StringLiteral(_)) => {
                            t == Type::String
                        }
                        (expressionNode::StringLiteral(_), expressionNode::Number(_)) => {
                            t == Type::String
                        }
                        _ => panic!("undexpected type"),
                    }
                }
                _ => false,
            };

            if !extype {
                panic!("Type not need {:?}, got {:?}", t, expr);
            }

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
                self.eat(TokenType::COLON);

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

        self.eat(TokenType::COLON);

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
                self.eat(TokenType::IntLiteral);
                let value = tok.get_value().parse::<i32>().unwrap();
                expressionNode::Number(numberNode::new(value))
            }
            TokenType::StringLiteral => {
                self.eat(TokenType::StringLiteral);
                expressionNode::StringLiteral(tok.get_value())
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
