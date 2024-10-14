/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
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

    /// Recursively inserts a value into the subtree rooted at this node.
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(mut left) = self.left.as_mut() {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(mut right) = self.right.as_mut() {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {} // Ignore duplicate values.
        }
    }

    /// Recursively searches for a value in the subtree rooted at this node.
    fn search(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search(value)),
            Ordering::Greater => self.right.as_ref().map_or(false, |right| right.search(value)),
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// Inserts a value into the BST.
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut root) => root.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    /// Searches for a value in the BST.
    fn search(&self, value: &T) -> bool {
        self.root.as_ref().map_or(false, |root| root.search(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // Check that an empty tree does not contain any value.
        assert_eq!(bst.search(&1), false);

        // Insert some values into the tree.
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // Check that all inserted values are present.
        assert_eq!(bst.search(&5), true);
        assert_eq!(bst.search(&3), true);
        assert_eq!(bst.search(&7), true);
        assert_eq!(bst.search(&2), true);
        assert_eq!(bst.search(&4), true);

        // Check that values not in the tree are not found.
        assert_eq!(bst.search(&1), false);
        assert_eq!(bst.search(&6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // Insert a value twice.
        bst.insert(1);
        bst.insert(1);

        // Check that the value is present.
        assert_eq!(bst.search(&1), true);

        // Check that there are no duplicate nodes.
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}