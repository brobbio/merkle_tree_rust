use crate::node::Node;
use crate::hash::Hash;


pub struct MerkleTree {
    root: Node,
    leaves: Vec<Node>,
}

impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree{
            root: None,
            leaves: Vec::new(),
        }
    }

    pub fn insert(&mut self, value:T){
        let leaf = Node::new_leaf(&value);
        self.leaves.push(leaf);
        self.compute();
    }


    pub fn compute(&mut self) {
        let mut level = self.leaves.clone()
        
        while level.len() > 1 {
            let mut level_minus_one = Vec::new();

            for (i,x) in level.iter().enumerate() {
                let mut left = x.clone();
                let right = if i < level.len() - 1 {
                    level[i+1].clone()
                } else {
                    x.clone()
                };
                level_minus_one.push(Node::new_internal(left, right))
            
            }

            level = level_minus_one;
        }
            
        
        self.root = level.pop();
        }

}