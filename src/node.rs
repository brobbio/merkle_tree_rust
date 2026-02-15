use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub hash: u64,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub value: Option<T>,
}

impl<T: Hash> Node<T> {
    pub fn new_leaf(value: T) -> Self {
        let hash = Self::hash_value(&value);

        Self {
            hash,
            left: None,
            right: None,
            value: Some(value),
        }
    }

    pub fn new_internal(left: Node<T>, right: Node<T>) -> Self {
        let combined = (left.hash, right.hash);
        let hash = Self::hash_value(&combined);

        Self {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            value: None,
        }
    }

    fn hash_value<V: Hash>(v: &V) -> u64 {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        hasher.finish()
    }
}
