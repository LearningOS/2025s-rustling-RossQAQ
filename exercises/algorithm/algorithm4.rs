/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if !self.search(value.clone()) {
            match self.root.as_deref_mut() {
                Some(root) => root.insert(value),
                None => self.root = Some(Box::new(TreeNode::new(value))),
            };
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match self.root.as_deref() {
            None => false,
            Some(root) => root.search(value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if self.value < value {
            match self.right.as_deref_mut() {
                None => self.right = Some(Box::new(Self::new(value))),
                Some(r) => r.insert(value),
            }
        } else if self.value > value {
            match self.left.as_deref_mut() {
                None => self.left = Some(Box::new(Self::new(value))),
                Some(l) => l.insert(value),
            }
        }
    }

    fn search(&self, value: T) -> bool {
        if self.value == value {
            true
        } else if self.value < value {
            self.right.as_deref().is_some_and(|r| r.search(value))
        } else {
            self.left.as_deref().is_some_and(|l| l.search(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
