
#[derive(Debug, Clone)]
pub struct variableNode{
    name : String,
    index : usize
}

impl variableNode{
    pub fn new(name : String, index : usize) -> Self{
        Self { 
            name,
            index 
        }
    }

    pub fn get_name(&self) -> String{
        return self.name.clone();
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}