use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn new_full(val: i32, left: TreeNode, right: TreeNode) -> Self {
        Self {
            val,
            left: Some(Rc::new(RefCell::new(left))),
            right: Some(Rc::new(RefCell::new(right))),
        }
    }
}

struct Solution;

impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = Vec::new();
        Self::push_node(&mut stack, root.as_ref());
        let mut min_diff = i32::MAX;
        if let Some(first) = stack.pop() {
            let mut pre = first.borrow().val;
            Self::push_node(&mut stack, first.borrow().right.as_ref());
            while let Some(node) = stack.pop() {
                let val = node.borrow().val;
                min_diff = min_diff.min(val - pre);
                pre = val;
                Self::push_node(&mut stack, node.borrow().right.as_ref());
            }
        }
        min_diff
    }

    fn push_node(stack: &mut Vec<Rc<RefCell<TreeNode>>>, node: Option<&Rc<RefCell<TreeNode>>>) {
        if let Some(inner) = node {
            stack.push(Rc::clone(inner));
            Self::push_node(stack, inner.borrow().left.as_ref());
        }
    }

    /// TODO: impl a non-recursive version
    // fn push_node(stack: &mut Vec<Rc<RefCell<TreeNode>>>, mut node: Option<&Rc<RefCell<TreeNode>>>) {
    //     while let Some(inner) = node {
    // 	    stack.push(Rc::clone(inner));
    // 	    node = inner.borrow().left.as_ref();
    // 	    //Rc::clone(node.borrow().left.as_ref().unwrap());
    //     }
    // }
}

#[test]
fn test_0783() {
    let tree = TreeNode::new_full(
        4,
        TreeNode::new_full(2, TreeNode::new(1), TreeNode::new(3)),
        TreeNode::new(6),
    );
    assert_eq!(
        1,
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(tree))))
    );
}
