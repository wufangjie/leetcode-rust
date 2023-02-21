use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while root.is_some() {
            match val.cmp(root.as_ref().unwrap().borrow()) {
                Ordering::Equal => root,
                Ordering::Less => root = root.unwrap().right,
                Ordering::Greater => root = root.unwrap().left,
            }
        }
        root
    }
}
