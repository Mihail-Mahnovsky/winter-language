use std::{result, string};

use crate::interpritator::objects::Object;
use crate::scopeNode;
use crate::expressionNode;

#[derive(Debug, Clone)]
pub struct functionNode{
    name : String,
    args : Vec<expressionNode>,
    scope : scopeNode,
    return_value : Object,
}

impl functionNode{
    pub fn new(name : String, args : Vec<expressionNode>,scope : scopeNode, return_value : Object) -> Self{
        Self { 
            name,
            args,
            scope,
            return_value, 
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_args(&self) -> Vec<expressionNode>{
        self.args.clone()
    }

    pub fn get_scope(&self) -> scopeNode{
        self.scope.clone()
    } 

    pub fn get_ret_val(&self) -> Object{
        self.get_ret_val().clone()
    }
}