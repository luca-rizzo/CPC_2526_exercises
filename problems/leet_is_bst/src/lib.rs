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
use std::rc::Rc;
use std::cell::RefCell;

struct Solution{}

impl Solution {

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::rec_is_bst(root, i32::MIN, i32::MAX)
    }

    fn rec_is_bst(root: Option<Rc<RefCell<TreeNode>>>, l_range: i32, r_range: i32) -> bool {
        let Some(root) = root else {
            return true;
        };
        let c_node = root.borrow();
        if !(c_node.val >= l_range && c_node.val < r_range) {
            return false;
        }
        Solution::rec_is_bst(c_node.left.clone(), l_range, c_node.val)
            && Solution::rec_is_bst(c_node.right.clone(), c_node.val, r_range)
    }
}
