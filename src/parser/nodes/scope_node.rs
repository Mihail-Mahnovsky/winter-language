use crate::interpritator::objects::Object;
use crate::parser::parser::Node;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct scopeNode {
    nodes: Vec<Node>,
    variables: RefCell<HashMap<String, Object>>,
}

impl scopeNode {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            nodes,
            variables: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }

    pub fn get_variables(&self) -> HashMap<String, Object> {
        self.variables.borrow().clone()
    }

    pub fn get_nodes_len(&mut self) -> usize {
        self.nodes.len()
    }

    pub fn get_variable(&self, name_of_var: String) -> Object {
        self.variables
            .borrow()
            .get(&name_of_var)
            .cloned()
            .unwrap_or_else(|| panic!("var '{}' not not obl", name_of_var))
    }

    pub fn add_variable(&self, name: String, obj: Object) {
        let mut variables = self.variables.borrow_mut();
        variables.insert(name, obj);
    }
}
