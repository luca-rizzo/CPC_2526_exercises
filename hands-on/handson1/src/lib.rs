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
    fn max_path_sum(tree: &Tree) -> i32 {
        let (max_sum, _) = rec_max_path_sum(&tree, Some(0));
        max_sum
    }

    fn rec_max_path_sum(tree: &Tree, id: Option<usize>) -> (i32, i32) {
        match id {
            None => (i32::MIN, 0),
            Some(id) => {
                assert!(id < tree.nodes.len(), "Node id is out of range");
                let c_node = &tree.nodes[id];
                let (l_max, l_max_path) = rec_max_path_sum(tree, c_node.id_left);
                let (r_max, r_max_path) = rec_max_path_sum(tree, c_node.id_right);
                let c_max = max(l_max_path + r_max_path + c_node.key, max(l_max, r_max));
                let path_max = max(l_max_path + c_node.key, r_max_path + c_node.key);
                (c_max, path_max)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_single_node() {
            let tree = Tree::with_root(5);
            assert_eq!(max_path_sum(&tree), 5);
        }

        #[test]
        fn test_two_leaves_simple() {
            let mut t = Tree::with_root(1);
            t.add_left_child(0, 2);
            t.add_right_child(0, 3);
            // path: 2→1→3 = 6
            assert_eq!(max_path_sum(&t), 6);
        }

        #[test]
        fn test_negative_branch() {
            let mut t = Tree::with_root(2);
            t.add_left_child(0, -1);
            t.add_right_child(0, 3);
            assert_eq!(max_path_sum(&t), 4);
        }

        #[test]
        fn test_negative_branch_single() {
            let mut t = Tree::with_root(2);
            t.add_left_child(0, -4);
            t.add_right_child(0, 3);
            assert_eq!(max_path_sum(&t), 3);
        }

        #[test]
        fn test_depth_two_tree() {
            let mut tree = Tree::with_root(-10);
            tree.add_left_child(0, 9);
            let id_r = tree.add_right_child(0, 20);
            tree.add_left_child(id_r, 15);
            tree.add_right_child(id_r, 7);
            // massimo path leaf-leaf: 15→20→7 = 42
            assert_eq!(max_path_sum(&tree), 42);
        }

        #[test]
        fn test_skewed_left() {
            let mut t = Tree::with_root(5);
            let l1 = t.add_left_child(0, 4);
            t.add_left_child(l1, 3);
            // una sola foglia (3) → nessuna coppia di foglie
            // policy: massimo path root-to-leaf = 5+4+3=12
            assert_eq!(max_path_sum(&t), 12);
        }

        #[test]
        fn test_complex_tree() {
            let mut t = Tree::with_root(10);
            let l = t.add_left_child(0, 2);
            let r = t.add_right_child(0, 10);
            t.add_left_child(l, 20);
            t.add_right_child(l, 1);
            let r_r = t.add_right_child(r, -5);
            t.add_left_child(r_r, 30);
            t.add_right_child(r_r, 4);
            // migliore path leaf-leaf: 20→2→10→10 = 42
            assert_eq!(max_path_sum(&t), 67);
        }

        #[test]
        fn test_with_negatives() {
            let mut t = Tree::with_root(-15);
            let l = t.add_left_child(0, 5);
            let r = t.add_right_child(0, 6);
            let l_l = t.add_left_child(l, -8);
            t.add_right_child(l, 1);
            let r_l = t.add_left_child(r, 3);
            t.add_right_child(r, 9);
            t.add_left_child(l_l, 2);
            t.add_right_child(l_l, 6);
            assert_eq!(max_path_sum(&t), 18);
        }
    }
}
