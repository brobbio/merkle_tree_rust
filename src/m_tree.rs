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

    pub fn root_hash(&self) {
        self.root.as_ref().map(|n| n.hash)
    }

    pub fn contains(& self, value: T) -> bool {
        let hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hasher;
            let mut h = DefaultHasher::new();
            value.hash(&mut h);
            h.finish();
        }

        self.leaves.iter.any(|n| n.hash == hash)
    }


    fn compute(&mut self) {
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