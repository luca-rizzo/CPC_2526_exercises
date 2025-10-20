//Definition for a binary tree node.
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
            right: None
        }
    }
}

struct Solution {}

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
impl Solution {

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (c_max, _) = Self::rec_max_path_sum(root);
        c_max
    }

    fn rec_max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (i32::MIN, 0),
            Some(node) => {
                let c_node = node.borrow();
                let (l_max, l_max_path) = Self::rec_max_path_sum(c_node.left.clone());
                let (r_max, r_max_path) = Self::rec_max_path_sum(c_node.right.clone());
                let l_max_path = max(0, l_max_path);
                let r_max_path = max(0, r_max_path);
                let through = l_max_path + r_max_path + c_node.val;
                // miglior percorso discendente estendibile
                let path_max = c_node.val + max(l_max_path, r_max_path);
                let c_max = max(through, max(l_max, r_max));
                (c_max, path_max)
            }
        }
    }
}