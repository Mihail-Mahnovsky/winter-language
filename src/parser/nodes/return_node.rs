use crate::expressionNode;

#[derive(Debug, Clone)]
pub struct returnNode {
    return_value: expressionNode,
}

impl returnNode {
    pub fn new(expr: expressionNode) -> Self {
        Self { return_value: expr }
    }

    pub fn get_return_value(&self) -> expressionNode {
        self.return_value.clone()
    }
}
