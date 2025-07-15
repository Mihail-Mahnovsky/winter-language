use crate::parser::parser::Type;

#[derive(Debug, Clone)]
pub struct variableNode{
    name : String,
    index : usize,
    typeS : Type,
}

impl variableNode{
    pub fn new(name : String, index : usize,typeS : Type) -> Self{
        Self { 
            name,
            index, 
            typeS,
        }
    }

    pub fn get_name(&self) -> String{
        return self.name.clone();
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_type(&self) -> Type{
        self.typeS.clone()
    }
}