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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut deque: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = VecDeque::new();

        if let Some(node) = &root {
            let node = node.clone();
            deque.push_back((node, 0));
            while let Some((n_rc, depth)) = deque.pop_front() {
                let n_ref = n_rc.borrow();
                if let Some(l_node) = &n_ref.left {
                    deque.push_back((l_node.clone(), depth + 1));
                }
                if let Some(r_node) = &n_ref.right {
                    deque.push_back((r_node.clone(), depth + 1));
                }
                if let Some(level_vec) = result.get_mut(depth) {
                    level_vec.push(n_ref.val);
                } else {
                    result.push(vec![n_ref.val]);
                }
            }
        }
        result
    }
}
