use crate::assignmentNode;
use crate::binOpNode;
use crate::parser::nodes::{expressionNode, numberNode};
use crate::variableNode;
use crate::echoNode;

use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Node {
    Assignment(assignmentNode),
    ExpressionNode(expressionNode),
    EchoNode(echoNode),
}

pub struct Parser {
    tokens_clone: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens_clone: Vec::new(),
            pos: 0,
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

    fn statement(&mut self) -> Node {
        let tok = self.current().clone();
        match tok.get_type() {
            TokenType::ID => {self.assignment()},
            TokenType::Echo => {
                self.eat(TokenType::Echo);
                let expr = self.expr();
                Node::EchoNode(echoNode::new(expr))
            },
            _ => Node::ExpressionNode(self.expr()),
        }
    }

    fn assignment(&mut self) -> Node {
        let name = self.current().get_value();
        //println!("Parsing assignment to variable: {}", name);
        self.eat(TokenType::ID);
        if self.current().get_type() == TokenType::Assignment {
            self.eat(TokenType::Assignment);
            let expr = self.expr();
            let var_node = variableNode::new(name);
            Node::Assignment(assignmentNode::new(var_node, expr))
        } else {
            panic!("Expected =");
        }
    }

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
                expressionNode::Variable(variableNode::new(name))
            },
            TokenType::LParen => {
                self.eat(TokenType::LParen);
                let expr = self.expr();
                self.eat(TokenType::RParen);
                expr
            },
            TokenType::ID => {
                self.eat(TokenType::ID);
                let name = tok.get_value();
                expressionNode::Variable(variableNode::new(name))
            },
            _ => panic!("Unexpected token in factor(xyeta): {:?}", tok),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Node {
        self.tokens_clone = tokens;
        self.pos = 0;
        self.statement()
    }
}