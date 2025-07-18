use crate::expressionNode;

#[derive(Debug, Clone)]
pub struct callNode {
    function_name: String,
    args: Vec<expressionNode>,
}

impl callNode {
    pub fn new(function_name: String, args: Vec<expressionNode>) -> Self {
        Self {
            function_name,
            args,
        }
    }

    pub fn get_function_name(&self) -> String {
        self.function_name.clone()
    }

    pub fn get_arguments(&self) -> Vec<expressionNode> {
        self.args.clone()
    }

    pub fn get_argument(&self, index: usize) -> Option<expressionNode> {
        self.args.get(index).cloned()
    }

    pub fn get_arguments_len(&self) -> usize {
        self.args.len()
    }
}
