use std::any::Any;
use std::collections::HashMap;
use std::thread::current;
use std::thread::panicking;
use std::vec;

use crate::assignmentNode;
use crate::binOpNode;
use crate::interpritator::objects::Object;
use crate::parser::nodes::{expressionNode, numberNode};
use crate::variableNode;
use crate::echoNode;
use crate::functionNode;
use crate::scopeNode;

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
pub enum Node {
    Assignment(assignmentNode),
    ExpressionNode(expressionNode),
    EchoNode(echoNode),
    FunctionNode(functionNode)
}

pub struct Parser {
    tokens_clone: Vec<Token>,
    pos: usize,
    variable_indices: HashMap<String, usize>,
    variable_types: HashMap<String, Type>,
    next_index: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens_clone: Vec::new(),
            pos: 0,
            variable_indices : HashMap::new(),
            variable_types : HashMap::new(),
            next_index : 0,
        }
    }

    fn current(&self) -> &Token {
        &self.tokens_clone[self.pos]
    }

    fn eat(&mut self, want_type: TokenType) {
        if self.pos < self.tokens_clone.len() && self.tokens_clone[self.pos].get_type() == want_type {
            self.pos += 1;
        } else {
            panic!("Unexpected XYETA:  {:?}, got {:?}", want_type, self.current().get_type());
        }
    }

    fn statement_list(&mut self) -> Vec<Node>{
        let mut nodes : Vec<Node> = Vec::new();

        while self.pos < self.tokens_clone.len() {
            if self.current().get_type() == TokenType::RBracket{
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
                if self.tokens_clone.get(self.pos + 1).map(|t| t.get_type()) == Some(TokenType::COLON) {
                    self.declaration()
                } else if self.tokens_clone.get(self.pos + 1).map(|t| t.get_type()) == Some(TokenType::Assignment) {
                    self.reassignment()
                } else {
                    panic!("Invalid statement starting with ID");
                }
            },
            TokenType::Echo => {
                self.eat(TokenType::Echo);
                let expr: expressionNode = self.expr();
                Node::EchoNode(echoNode::new(expr))
            },
            //TokenType::Fn => { self.custom_func() }
            _ => Node::ExpressionNode(self.expr()),
        }
    }

    fn allocate_variable(&mut self, name: String) -> usize {
        if let Some(&idx) = self.variable_indices.get(&name) {
            idx
        } else {
            let idx = self.next_index;
            self.variable_indices.insert(name, idx);
            self.next_index += 1;
            idx
        }
    }


    fn reassignment(&mut self) -> Node {
        let name = self.current().get_value();
        self.eat(TokenType::ID);
        self.eat(TokenType::Assignment);

        let expr = self.expr();

        let index = *self.variable_indices.get(&name)
            .unwrap_or_else(|| panic!("nannot assign to not realized variable '{}'", name));

        let t = self.variable_types.get(&name)
            .unwrap_or_else(|| panic!("no type found for variable '{}'", name)).clone();

        let var_node = variableNode::new(name, index, t);
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

        if self.variable_indices.contains_key(&name) {
            panic!("Variable '{}' already declared", name);
        }

        self.variable_types.insert(name.clone(), t.clone());
        self.eat(self.current().get_type()); // съедаем тип (IntType и т.д.)

        if self.current().get_type() == TokenType::Assignment {
            self.eat(TokenType::Assignment);
            let expr = self.expr();

            let extype = match &expr {
                expressionNode::Number(_) => t == Type::Int,
                expressionNode::StringLiteral(_) => t == Type::String,
                expressionNode::DefaultValue(_) => true,
                expressionNode::Variable(var) => var.get_type() == t,
                _ => false,
            };

            if !extype {
                panic!("Type not need {:?}, got {:?}", t, expr);
            }

            let index = self.allocate_variable(name.clone());
            return Node::Assignment(assignmentNode::new(
                variableNode::new(name, index, t),
                expr,
            ));
        }

        let index = self.allocate_variable(name.clone());
        Node::Assignment(assignmentNode::new(
            variableNode::new(name, index, t.clone()),
            expressionNode::DefaultValue(t),
        ))
    }



    fn parse_scope(&mut self) -> scopeNode{
        self.eat(TokenType::LBracket);
        let mut nodes : Vec<Node> = Vec::new();
        nodes = self.statement_list();
        self.eat(TokenType::RBracket);
        scopeNode::new(nodes)
    }

    /*fn custom_func(&mut self) -> Node{
        self.eat(TokenType::Fn);
        if self.current().get_type() == TokenType::ID {
            let name = self.current().get_value();
            self.eat(TokenType::ID);
            let mut args : Vec<Object> = Vec::new(); 
            self.eat(TokenType::LParen);
            for tok in self.tokens_clone{
                if tok.get_type() == TokenType::ID{
                    args.push();
                    self.eat(TokenType::Coma);
                }
                break;
            }
            self.eat(TokenType::RParen);
            let mut return_val : Object;
            match self.current().get_value().as_str() {
                "string"=>{return_val = Object::String((String::new())); self.eat(TokenType::Type);},
                "int"=>{return_val = Object::Int((0)); self.eat(TokenType::Type);},
                "bool"=>{return_val = Object::Bool((false)); self.eat(TokenType::Type);},
                "void"=>{return_val = Object::Void();self.eat(TokenType::Type);},
                _=>panic!("non type")
            }


            self.eat(TokenType::RBracket);
            return Node::EchoNode((echoNode::new(expressionNode::BinOp(binOpNode::new(2, 2, '=')))));
        } 
        panic!("xyeta a ne token : {}", self.current().get_value())
    }*/


    fn expr(&mut self) -> expressionNode {
        let left = self.term();
        self.term_tail(left)
    }

    fn term_tail(&mut self, mut left: expressionNode) -> expressionNode {
        while self.pos < self.tokens_clone.len() {
            let tok = self.current().clone();
            if tok.get_type() == TokenType::Operator && (tok.get_value() == "+" || tok.get_value() == "-") {
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
            if tok.get_type() == TokenType::Operator && (tok.get_value() == "*" || tok.get_value() == "/") {
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
            },
            TokenType::StringLiteral => {
                self.eat(TokenType::StringLiteral);
                expressionNode::StringLiteral(tok.get_value())
            },
            TokenType::ID => {
                let name = self.current().get_value();
                self.eat(TokenType::ID);
                let index = if let Some(&idx) = self.variable_indices.get(&name) { idx } else {
                    panic!("var '{}' not obivlena", name);
                };
                let var_type = if let Some(t) = self.variable_types.get(&name) {
                    t.clone()
                } else {
                    panic!("var '{}' not obivlena", name);
                };
                

                expressionNode::Variable(variableNode::new(name,index,var_type))
            },
            TokenType::LParen => {
                self.eat(TokenType::LParen);
                let expr = self.expr();
                self.eat(TokenType::RParen);
                expr
            },
            _ => panic!("Unexpected token in factor(xyeta): {:?}", tok),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Vec<Node> {
        self.tokens_clone = tokens;
        self.pos = 0;
        self.statement_list()
    }
}