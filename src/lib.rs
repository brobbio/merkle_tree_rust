pub mod node;
pub mod m_tree;

pub use m_tree::MerkleTree;

mod tests {
    use crate::m_tree::MerkleTree; 

    #[test]
    fn test_empty_tree() {
        let tree: MerkleTree<i32> = MerkleTree::new();
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_insert_one() {
        let mut tree = MerkleTree::new();

        tree.insert(10);

        assert_eq!(tree.len(), 1);
        assert!(tree.root().is_some());
    }

    #[test]
    fn test_insert_multiple() {
        let mut tree = MerkleTree::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn test_root_changes() {
        let mut tree = MerkleTree::new();

        tree.insert(1);
        let r1 = tree.root_hash();

        tree.insert(2);
        let r2 = tree.root_hash();

        assert_ne!(r1, r2);
    }
}
