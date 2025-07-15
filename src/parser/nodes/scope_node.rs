use crate::parser::parser::Node;

#[derive(Debug, Clone)]
pub struct scopeNode{
    nodes : Vec<Node>
}

impl scopeNode{
    pub fn new(nodes : Vec<Node>) -> Self{
        Self { 
            nodes
         }
    }

    pub fn get_nodes(&self) -> Vec<Node>{
        self.nodes.clone()
    }

    pub fn get_nodes_len(&self) -> usize{
        self.nodes.len()
    }

    pub fn get_node_by_index(&self,index : usize) -> Node{
        self.nodes[index].clone()
    }
}