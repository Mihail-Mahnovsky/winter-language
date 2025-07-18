use crate::parser::parser::Node;
use crate::interpritator::objects::Object;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct scopeNode {
    nodes: Vec<Node>,
    variables: HashMap<String, Object>,
}

impl scopeNode {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            nodes,
            variables: HashMap::new(),
        }
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }

    pub fn get_nodes_len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_variable(&self, name_of_var: String) -> Object {
        self.variables.get(&name_of_var).cloned().unwrap_or_else(|| {
            panic!("var '{}' not not obl", name_of_var)
        })
    }

    pub fn add_variable(&mut self, name: String, obj: Object) {
        self.variables.insert(name, obj);
    }
}
