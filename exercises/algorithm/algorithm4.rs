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
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) { // self: &mut BinarySearchTree
        //TODO
        // fn search(root: &Option<Box<TreeNode<T>>>, value: &T) -> bool
        if TreeNode::search(&self.root, &value) { // 检查是否有重复值
            return ();
        }
        
        if let Some(root) = &mut self.root {
            // root: &mut Box<TreeNode<T>>
            let t = root.as_mut(); // t: &mut TreeNode<T>
            t.insert(value);
        } else { // 空树直接插入
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool { // self: &BinarySearchTree
        //TODO
        // fn search(root: &Option<Box<TreeNode<T>>>, value: &T) -> bool
        TreeNode::search(&self.root, &value) // 查找元素
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) { // self: &mut TreeNode<T>
        //TODO
        if self.value < value {
            if let Some(left) = &mut self.left {
                // left: &mut Box<TreeNode<T>>
                left.as_mut().insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.as_mut().insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    fn search(root: &Option<Box<TreeNode<T>>>, value: &T) -> bool { // self: &TreeNode<T>
        if let Some(node) = root {
            // node: &Box<TreeNode<T>>
            if node.value == *value { // (*node).as_ref().value == *value
                true
            } else if node.value < *value {
                //TreeNode::search(&(*node).as_ref().left, value)
                TreeNode::search(&(node.left), value)
            } else {
                TreeNode::search(&(node.right), value)
            }
        } else { // 空树
            false
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    
