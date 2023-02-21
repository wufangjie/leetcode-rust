use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::insert_rec(&mut root, val);
        root
    }

    fn insert_rec(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if node.is_none() {
            *node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        } else {
            let cur_val = node.as_ref().unwrap().borrow().val;
            if cur_val < val {
                Solution::insert_rec(&mut node.as_mut().unwrap().borrow_mut().right, val);
            } else {
                Solution::insert_rec(&mut node.as_mut().unwrap().borrow_mut().left, val);
            }
        }
    }
}
