use crate::scopeNode;
use crate::expressionNode;
use crate::parser::parser::Arg;

#[derive(Debug, Clone)]
pub struct function{
    name : String,
    scope : scopeNode,
    args : Vec<Arg>,
    return_val : expressionNode,
}

impl function{
    pub fn new(name : String, scope : scopeNode, args : Vec<Arg>,return_val : expressionNode) -> Self{
        Self {
            name,
            scope,
            args,
            return_val,
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_scope(&self) -> scopeNode{
        self.scope.clone()
    }

    pub fn get_args(&self) -> Vec<Arg>{
        self.args.clone()
    }

    pub fn get_return_value(&self) -> expressionNode{
        self.return_val.clone()
    }
}

//impl Clone for function {
   // fn clone(&self) -> Self {
     //   Self::new(self.name, self.scope.clone(), args.clone(), self.return_val)
    //}
//}