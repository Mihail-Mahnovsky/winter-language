
#[derive(Debug, Clone)]
pub struct variableNode{
    name : String
}

impl variableNode{
    pub fn new(name : String) -> Self{
        Self { 
            name, 
        }
    }

    pub fn get_name(&self) -> String{
        return self.name.clone();
    }
}