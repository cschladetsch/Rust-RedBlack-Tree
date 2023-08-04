#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();

        assert!(!tree.search(5));

        tree.insert(5);
        tree.insert(3);
        tree.insert(8);

        assert!(tree.search(5));
        assert!(tree.search(3));
        assert!(tree.search(8));
        assert!(!tree.search(10));
    }

    #[test]
    fn test_remove() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();

        tree.insert(5);
        tree.insert(3);
        tree.insert(8);
        tree.insert(2);
        tree.insert(4);
        tree.insert(7);

        assert!(tree.search(5));
        tree.remove(5);
        assert!(!tree.search(5));

        assert!(tree.search(3));
        tree.remove(3);
        assert!(!tree.search(3));

        assert!(tree.search(8));
        tree.remove(8);
        assert!(!tree.search(8));
    }
}

