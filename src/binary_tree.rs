use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn add_left(&mut self, node: TreeNode) {
        self.left = Some(Rc::new(RefCell::new(node)))
    }

    pub fn add_right(&mut self, node: TreeNode) {
        self.right = Some(Rc::new(RefCell::new(node)))
    }
}

type OptNode = Option<Rc<RefCell<TreeNode>>>;
pub fn is_valid(root: OptNode) -> bool {
    // check the left, if it is less than the minimum, return true
    // check the right, if it is greater than the max, return true
    // if you reach none, return true
    valid(&root, i32::MIN, i32::MAX)
}

fn valid(root: &OptNode, minimum: i32, maximum: i32) -> bool {
    // assuming the limits are within 2^32 - 1
    match root {
        None => true,
        Some(n) => {
            let node = n.borrow();
            if !(node.val > minimum && node.val < maximum) {
                return false;
            }

            valid(&node.left, minimum, node.val) && valid(&node.right, node.val, maximum)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_binary_search_tree_test_1() {
        let mut root = TreeNode::new(2);
        root.add_left(TreeNode::new(1));
        root.add_right(TreeNode::new(3));

        assert!(is_valid(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn valid_binary_search_tree_test_2() {
        let mut root = TreeNode::new(5);
        root.add_left(TreeNode::new(1));

        let mut right = TreeNode::new(4);
        right.add_left(TreeNode::new(3));
        right.add_right(TreeNode::new(6));

        root.add_right(right);

        assert!(!is_valid(Some(Rc::new(RefCell::new(root)))));
    }
}
