use crate::node::Node;


pub struct MerkleTree<T: std::fmt::Debug> {
    root: Option<Node<T>>,
    leaves: Vec<Node<T>>,
}

impl<T: std::hash::Hash + Clone + std::fmt::Debug> MerkleTree<T> {
    pub fn new() -> Self {
        MerkleTree{
            root: None,
            leaves: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        return self.leaves.len();
    }

    pub fn root(&self) -> Option<&Node<T>> {
        return self.root.as_ref();
    }

    pub fn insert(&mut self, value:T){
        let leaf = Node::new_leaf(value);
        self.leaves.push(leaf);
        self.compute();
    }

    pub fn root_hash(&self) -> Option<u64> {
        //dbg!(self.root.as_ref().map(|n| n.hash));
        return self.root.as_ref().map(|n| n.hash);
    }

    pub fn contains(& self, value: T) -> bool {
        let hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hasher;
            let mut h = DefaultHasher::new();
            value.hash(&mut h);
            h.finish()
        };

        return self.leaves.iter().any(|n| n.hash == hash);
    }


    fn compute(&mut self) {
        let mut level = self.leaves.clone();
        
        
        while level.len() > 1 {
            
            let mut level_minus_one = Vec::new();

            let mut i = 0;

            while i < level.len() {
                let left = level[i].clone();
                let right = if i < level.len() - 1 {
                    level[i+1].clone()
                } else {
                    level[i].clone()
                };

                let parent = Node::new_internal(left, right);
                level_minus_one.push(parent);
                i += 2;
            
            }

            level = level_minus_one;
        }
            
        
        self.root = level.pop();
    }

}