use crate::expressionNode;
use crate::parser::parser::{Arg, Type};
use crate::scopeNode;

#[derive(Debug, Clone)]
pub struct function {
    name: String,
    scope: scopeNode,
    args: Vec<Arg>,
    return_val: Type,
}

impl function {
    pub fn new(name: String, scope: scopeNode, args: Vec<Arg>, return_val: Type) -> Self {
        Self {
            name,
            scope,
            args,
            return_val,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_scope(&self) -> scopeNode {
        self.scope.clone()
    }

    pub fn get_scope_mut(&mut self) -> &mut scopeNode {
        &mut self.scope
    }

    pub fn get_args(&self) -> Vec<Arg> {
        self.args.clone()
    }

    pub fn get_return_value(&self) -> Type {
        self.return_val.clone()
    }
}
