// =========================================================
// tree module: it contains the tree data structure provided
// ==========================================================
mod tree {
    pub struct Node {
        pub key: i32,
        pub id_left: Option<usize>,
        pub id_right: Option<usize>,
    }

    impl Node {
        fn new(key: i32) -> Self {
            Self {
                key,
                id_left: None,
                id_right: None,
            }
        }
    }

    pub struct Tree {
        pub nodes: Vec<Node>,
    }

    impl Tree {
        pub fn with_root(key: i32) -> Self {
            Self {
                nodes: vec![Node::new(key)],
            }
        }

        pub fn add_left_child(&mut self, parent_id: usize, key: i32) -> usize {
            self.add_node(parent_id, key, true)
        }

        pub fn add_right_child(&mut self, parent_id: usize, key: i32) -> usize {
            self.add_node(parent_id, key, false)
        }

        /// Adds a child to the node with `parent_id` and returns the id of the new node.
        /// The new node has the specified `key`. The new node is the left  child of the
        /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
        ///
        /// # Panics    ///  if the `parent_id` does not exist, or if the node `parent_id ` has
        /// the child already set.
        pub fn add_node(&mut self, parent_id: usize, key: i32, is_left: bool) -> usize {
            assert!(
                parent_id < self.nodes.len(),
                "Parent node id does not exist"
            );
            if is_left {
                assert_eq!(
                    self.nodes[parent_id].id_left, None,
                    "Parent node has the left child already set"
                );
            } else {
                assert_eq!(
                    self.nodes[parent_id].id_right, None,
                    "Parent node has the right child already set"
                );
            }

            let child_id = self.nodes.len();
            self.nodes.push(Node::new(key));

            let child = if is_left {
                &mut self.nodes[parent_id].id_left
            } else {
                &mut self.nodes[parent_id].id_right
            };

            *child = Some(child_id);

            child_id
        }
    }
}

// ===============================================================================
// Exercise 1: Write a method to check if the binary tree is a Binary Search Tree
// ===============================================================================
mod is_bst {
    use crate::tree::Tree;

    fn is_bst(tree: &Tree) -> bool {
        rec_is_bst(tree, Some(0), i32::MIN, i32::MAX)
    }

    fn rec_is_bst(tree: &Tree, id: Option<usize>, l_range: i32, r_range: i32) -> bool {
        match id {
            None => true,
            Some(id) => {
                assert!(id < tree.nodes.len(), "Node id is out of range");
                let c_node = &tree.nodes[id];
                if !(c_node.key >= l_range && c_node.key < r_range) {
                    return false;
                }
                rec_is_bst(&tree, c_node.id_left, l_range, c_node.key)
                    && rec_is_bst(&tree, c_node.id_right, c_node.key, r_range)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_depth_one_tree() {
            let mut tree = Tree::with_root(10);
            tree.add_left_child(0, 5);
            tree.add_right_child(0, 22);
            assert!(is_bst(&tree));
        }

        #[test]
        fn test_depth_two_tree() {
            let mut tree = Tree::with_root(10);
            let id_left = tree.add_left_child(0, 5);
            tree.add_left_child(id_left, 4);
            tree.add_right_child(id_left, 8);
            let id_right = tree.add_right_child(0, 22);
            tree.add_left_child(id_right, 12);
            tree.add_right_child(id_right, 30);
            assert!(is_bst(&tree));
        }

        #[test]
        fn test_typical_counter_example() {
            let mut tree = Tree::with_root(10);
            let id_left = tree.add_left_child(0, 5);
            tree.add_right_child(id_left, 12);
            assert!(!is_bst(&tree));
        }

        #[test]
        fn test_complex_true_bst_not_full() {
            let mut tree = Tree::with_root(15);

            // Left subtree
            let id_l = tree.add_left_child(0, 7);
            let id_l_l = tree.add_left_child(id_l, 3);
            let id_l_r = tree.add_right_child(id_l, 9);

            tree.add_left_child(id_l_l, 2);
            tree.add_right_child(id_l_l, 5);

            tree.add_left_child(id_l_r, 8);
            tree.add_right_child(id_l_r, 10);

            // Right subtree
            let id_r = tree.add_right_child(0, 23);
            let id_r_l = tree.add_left_child(id_r, 19);
            tree.add_right_child(id_r, 31);

            tree.add_left_child(id_r_l, 17);
            tree.add_right_child(id_r_l, 21);

            assert!(is_bst(&tree));
        }

        #[test]
        fn test_complex_true_bst() {
            let mut tree = Tree::with_root(15);

            // Left subtree
            let id_l = tree.add_left_child(0, 7);
            let id_l_l = tree.add_left_child(id_l, 3);
            let id_l_r = tree.add_right_child(id_l, 9);

            tree.add_left_child(id_l_l, 2);
            tree.add_right_child(id_l_l, 5);

            tree.add_left_child(id_l_r, 8);
            tree.add_right_child(id_l_r, 10);

            // Right subtree
            let id_r = tree.add_right_child(0, 23);
            let id_r_l = tree.add_left_child(id_r, 19);
            let id_r_r = tree.add_right_child(id_r, 31);

            tree.add_left_child(id_r_l, 17);
            tree.add_right_child(id_r_l, 21);

            tree.add_left_child(id_r_r, 29);
            tree.add_right_child(id_r_r, 41);

            assert!(is_bst(&tree));
        }

        #[test]
        fn test_complex_false_bst_ancestor_violation() {
            let mut tree = Tree::with_root(15);

            // Left subtree
            let id_l = tree.add_left_child(0, 7);
            let id_l_l = tree.add_left_child(id_l, 3);
            let id_l_r = tree.add_right_child(id_l, 9);

            tree.add_left_child(id_l_l, 2);
            tree.add_right_child(id_l_l, 5);

            tree.add_left_child(id_l_r, 8);
            tree.add_right_child(id_l_r, 10);

            // Right subtree with the violation
            let id_r = tree.add_right_child(0, 23);
            let id_r_l = tree.add_left_child(id_r, 19);
            let id_r_r = tree.add_right_child(id_r, 31);

            // Here's the subtle violation (should be > 15 because it's in root's right subtree)
            tree.add_left_child(id_r_l, 14);
            tree.add_right_child(id_r_l, 21);

            tree.add_left_child(id_r_r, 29);
            tree.add_right_child(id_r_r, 41);

            assert!(!is_bst(&tree));
        }
    }
}

// ===============================================================================
// Exercise 2: Write a method to solve the Maximum Path Sum problem
// The method must return the sum of the maximum simple path connecting two leaves
// ===============================================================================
mod max_path_sum_leaf {
    use crate::tree::Tree;
    use std::cmp::max;

    struct MpsRes(i32, i32);

    fn max_path_sum(tree: &Tree) -> i32 {
        let MpsRes(between_leaves, _) = rec_max_path_sum(tree, Some(0));
        between_leaves
    }

    fn rec_max_path_sum(tree: &Tree, id: Option<usize>) -> MpsRes {
        let Some(id) = id else {
            return MpsRes(i32::MIN, i32::MIN);
        };
        assert!(id < tree.nodes.len(), "Node id is out of range");
        let c_node = &tree.nodes[id];
        let MpsRes(l_max, l_max_path) = rec_max_path_sum(tree, c_node.id_left);
        let MpsRes(r_max, r_max_path) = rec_max_path_sum(tree, c_node.id_right);
        let sub_tree_max = max(l_max, r_max);
        // Update the maximum only if there is a valid path between two leaves passing through the current node
        match (c_node.id_left.is_some(), c_node.id_right.is_some()) {
            (true, true) => {
                // In this case, it's possible to update the maximum because there exists
                // a path between two leaves that passes through the current node.
                let best_down = c_node.key + max(l_max_path, r_max_path);
                let through = max(sub_tree_max, l_max_path + r_max_path + c_node.key);
                MpsRes(through, best_down)
            }
            (true, false) => MpsRes(sub_tree_max, c_node.key + l_max_path),
            (false, true) => MpsRes(sub_tree_max, c_node.key + r_max_path),
            (false, false) => MpsRes(sub_tree_max, c_node.key),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_single_node() {
            let tree = Tree::with_root(5);
            assert_eq!(max_path_sum(&tree), i32::MIN);
        }

        #[test]
        fn test_two_positives_leaves() {
            let mut t = Tree::with_root(1);
            t.add_left_child(0, 2);
            t.add_right_child(0, 3);
            assert_eq!(max_path_sum(&t), 6);
        }

        #[test]
        fn test_two_negatives_leaves() {
            let mut t = Tree::with_root(1);
            t.add_left_child(0, -12);
            t.add_right_child(0, -3);
            assert_eq!(max_path_sum(&t), -14);
        }

        #[test]
        fn test_single_positive_branch() {
            let mut t = Tree::with_root(2);
            let l = t.add_left_child(0, 5);
            t.add_left_child(l, 3);
            assert_eq!(max_path_sum(&t), i32::MIN);
        }

        #[test]
        fn test_single_negative_branch() {
            let mut t = Tree::with_root(-2);
            let l = t.add_left_child(0, -5);
            t.add_left_child(l, -3);
            assert_eq!(max_path_sum(&t), i32::MIN);
        }

        #[test]
        fn test_not_full_left_subtree() {
            let mut t = Tree::with_root(2);
            let l = t.add_left_child(0, -10);
            t.add_right_child(0, 3);
            t.add_left_child(l, -8);
            assert_eq!(max_path_sum(&t), -13);
        }

        #[test]
        fn test_not_full_right_subtree() {
            let mut t = Tree::with_root(2);
            let r = t.add_left_child(0, -10);
            t.add_right_child(0, 3);
            t.add_left_child(r, -8);
            assert_eq!(max_path_sum(&t), -13);
        }

        #[test]
        fn test_depth_two_tree() {
            let mut tree = Tree::with_root(2);
            let l = tree.add_left_child(0, -10);
            tree.add_right_child(0, 3);
            tree.add_left_child(l, -8);
            tree.add_right_child(l, 7);
            assert_eq!(max_path_sum(&tree), 2);
        }

        #[test]
        fn test_depth_two_full_tree_max_through_root() {
            let mut tree = Tree::with_root(6);
            let l = tree.add_left_child(0, -10);
            let r = tree.add_right_child(0, 13);
            tree.add_left_child(l, -3);
            tree.add_right_child(l, 7);
            tree.add_left_child(r, -1);
            tree.add_right_child(r, 12);
            assert_eq!(max_path_sum(&tree), 28);
        }

        #[test]
        fn test_depth_two_full_tree_max_in_left_sub_tree() {
            let mut tree = Tree::with_root(6);
            let l = tree.add_left_child(0, -10);
            let r = tree.add_right_child(0, 13);
            tree.add_left_child(l, -3);
            tree.add_right_child(l, 7);
            tree.add_left_child(r, 3);
            tree.add_right_child(r, 22);
            assert_eq!(max_path_sum(&tree), 38);
        }

        #[test]
        fn test_depth_two_full_tree_max_in_right_sub_tree() {
            let mut tree = Tree::with_root(-16);
            let l = tree.add_left_child(0, 10);
            let r = tree.add_right_child(0, 2);
            tree.add_left_child(l, 3);
            tree.add_right_child(l, 7);
            tree.add_left_child(r, 3);
            tree.add_right_child(r, 2);
            assert_eq!(max_path_sum(&tree), 20);
        }

        #[test]
        fn test_complex_tree_max_in_subtree() {
            let mut t = Tree::with_root(-10);

            // Struttura Sinistra
            let l = t.add_left_child(0, 5);
            let l_l = t.add_left_child(l, -6);
            let l_r = t.add_right_child(l, 1);

            let l_l_l = t.add_left_child(l_l, 2);
            let l_l_r = t.add_right_child(l_l, 6);

            t.add_left_child(l_r, 3);
            t.add_right_child(l_r, 2);

            let r = t.add_right_child(0, 6);
            let r_l = t.add_left_child(r, 3);
            let r_r = t.add_right_child(r, 4);

            t.add_left_child(r_l, 0);

            let r_r_r = t.add_right_child(r_r, 2);
            t.add_left_child(r_r_r, 0);

            let r_r_r_r = t.add_right_child(r_r_r, -1);
            let r_r_r_r_l = t.add_left_child(r_r_r_r, 10);
            t.add_right_child(r_r_r_r_l, 2);

            assert_eq!(max_path_sum(&t), 26);
        }

        #[test]
        fn test_complex_tree_max_through_root() {
            let mut t = Tree::with_root(10);

            // Struttura Sinistra
            let l = t.add_left_child(0, 5);
            let l_l = t.add_left_child(l, -6);
            let l_r = t.add_right_child(l, 1);

            let l_l_l = t.add_left_child(l_l, 2);
            let l_l_r = t.add_right_child(l_l, 6);

            t.add_left_child(l_r, 3);
            t.add_right_child(l_r, 2);

            let r = t.add_right_child(0, 6);
            let r_l = t.add_left_child(r, 3);
            let r_r = t.add_right_child(r, 4);

            t.add_left_child(r_l, 0);

            let r_r_r = t.add_right_child(r_r, 2);
            t.add_left_child(r_r_r, 0);

            let r_r_r_r = t.add_right_child(r_r_r, -1);
            let r_r_r_r_l = t.add_left_child(r_r_r_r, 10);
            t.add_right_child(r_r_r_r_l, 2);

            assert_eq!(max_path_sum(&t), 45);
        }

    }
}
