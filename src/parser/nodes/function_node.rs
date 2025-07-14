use std::result;

use crate::interpritator::objects::Object;
use crate::scopeNode;

#[derive(Debug, Clone)]
pub struct functionNode{
    args : Vec<Object>,
    scope : scopeNode,
    return_value : Object,
}

impl functionNode{
    pub fn new(args : Vec<Object>,scope : scopeNode, return_value : Object) -> Self{
        Self { 
            args,
            scope,
            return_value, 
        }
    }

    pub fn get_args(&self) -> Vec<Object>{
        self.args.clone()
    }

    pub fn get_scope(&self) -> scopeNode{
        self.scope.clone()
    } 

    pub fn get_ret_val(&self) -> Object{
        self.get_ret_val().clone()
    }
}