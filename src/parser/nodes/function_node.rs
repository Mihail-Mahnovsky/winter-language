use std::{result, string};

use crate::expressionNode;
use crate::parser::parser::{Arg, Type};
use crate::scopeNode;

#[derive(Debug, Clone)]
pub struct functionNode {
    name: String,
    args: Vec<Arg>,
    scope: scopeNode,
    return_value: Type,
}

impl functionNode {
    pub fn new(name: String, args: Vec<Arg>, scope: scopeNode, return_value: Type) -> Self {
        Self {
            name,
            args,
            scope,
            return_value,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_args(&self) -> Vec<Arg> {
        self.args.clone()
    }

    pub fn get_scope(&self) -> scopeNode {
        self.scope.clone()
    }

    pub fn get_ret_val(&self) -> Type {
        self.return_value.clone()
    }
}
