use crate::parser::parser::Type;

#[derive(Debug, Clone)]
pub struct variableNode{
    name : String,
    typeS : Type,
}

impl variableNode{
    pub fn new(name : String,typeS : Type) -> Self{
        Self { 
            name,
            typeS,
        }
    }

    pub fn get_name(&self) -> String{
        return self.name.clone();
    }

    pub fn get_type(&self) -> Type{
        self.typeS.clone()
    }
}